use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::{Constraint, Direction, Layout, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use std::fmt;
use std::option::Option;

type Coordinate = (usize, usize);

pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Player {
    #[default]
    X,
    O,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

type Owner = Option<Player>;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Winner {
    #[default]
    NoWinner,
    CatsGame,
    Player(Player),
}

type Row = [Owner; 3];
type Board = [Row; 3];

trait Boardable {
    fn get_at(&self, coordinate: Coordinate) -> Owner;
    fn set_at(&self, coordinate: Coordinate, owner: Owner) -> Board;
}

impl Boardable for Board {
    fn get_at(&self, (row_idx, col_idx): Coordinate) -> Owner {
        self[row_idx][col_idx].clone()
    }

    fn set_at(&self, (row_idx, col_idx): Coordinate, owner: Owner) -> Board {
        let mut next: Board = self.clone();
        next[row_idx][col_idx] = owner;
        next
    }
}

#[derive(Debug, Default)]
pub struct GameState {
    current_player: Player,
    board: Board,
    active_cell: Coordinate,
}

struct Cell<'game_state> {
    game_state: &'game_state GameState,
    coordinate: Coordinate,
}

impl<'game_state> Cell<'game_state> {
    fn new(game_state: &'game_state GameState, coordinate: Coordinate) -> Self {
        Self {
            game_state,
            coordinate,
        }
    }

    fn is_active_cell(&self) -> bool {
        let active_cell = &self.game_state.active_cell;
        active_cell.0 == self.coordinate.0 && active_cell.1 == self.coordinate.1
    }

    fn owner(&self) -> Owner {
        self.game_state.board.get_at(self.coordinate)
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    fn reset(&mut self) {
        self.board = Board::default()
    }

    fn cell(&self, coordinate: Coordinate) -> Cell<'_> {
        Cell::new(self, coordinate)
    }

    pub fn handle_on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Char('h') | KeyCode::Left) => self.move_active_cell(Dir::Left),
            (_, KeyCode::Char('j') | KeyCode::Down) => self.move_active_cell(Dir::Down),
            (_, KeyCode::Char('k') | KeyCode::Up) => self.move_active_cell(Dir::Up),
            (_, KeyCode::Char('l') | KeyCode::Right) => self.move_active_cell(Dir::Right),
            (_, KeyCode::Enter) => self.make_move(),
            (_, KeyCode::Char('r')) => self.reset(),
            _ => {}
        }
    }

    pub fn move_active_cell(&mut self, direction: Dir) {
        match direction {
            Dir::Left => {
                if self.active_cell.1 > 0 {
                    let next_active_cell = (self.active_cell.0, self.active_cell.1 - 1);
                    self.active_cell = next_active_cell;
                }
            }
            Dir::Right => {
                if self.active_cell.1 < 2 {
                    let next_active_cell = (self.active_cell.0, self.active_cell.1 + 1);
                    self.active_cell = next_active_cell;
                }
            }
            Dir::Up => {
                if self.active_cell.0 > 0 {
                    let next_active_cell = (self.active_cell.0 - 1, self.active_cell.1);
                    self.active_cell = next_active_cell;
                }
            }
            Dir::Down => {
                if self.active_cell.0 < 2 {
                    let next_active_cell = (self.active_cell.0 + 1, self.active_cell.1);
                    self.active_cell = next_active_cell;
                }
            }
        }
    }

    pub fn make_move(&mut self) {
        if self.is_valid_move() {
            self.set_cell(self.active_cell.clone(), self.current_player.clone());
            self.toggle_current_player()
        }
    }

    fn is_valid_move(&mut self) -> bool {
        let is_current_cell_empty = self.board.get_at(self.active_cell) == Option::None;
        let is_game_over = self.winner() != Winner::NoWinner;
        is_current_cell_empty && !is_game_over
    }

    fn set_cell(&mut self, active_cell: Coordinate, player: Player) {
        let owner = Option::Some(player);
        let next_board = self.board.set_at(active_cell, owner);
        self.board = next_board;
    }

    fn toggle_current_player(&mut self) {
        match self.current_player {
            Player::O => self.current_player = Player::X,
            Player::X => self.current_player = Player::O,
        }
    }

    pub fn winner(&self) -> Winner {
        let lines = [
            [(0, 0), (0, 1), (0, 2)], // row 0
            [(1, 0), (1, 1), (1, 2)], // row 1
            [(2, 0), (2, 1), (2, 2)], // row 2
            [(0, 0), (1, 0), (2, 0)], // col 0
            [(0, 1), (1, 1), (2, 1)], // col 1
            [(0, 2), (1, 2), (2, 2)], // col 2
            [(0, 0), (1, 1), (2, 2)], // diag left to right
            [(0, 2), (1, 1), (2, 0)], // diag right to left
        ];

        let is_x_winner = lines.into_iter().any(|line| {
            line.into_iter()
                .map(|coord| self.board.get_at(coord))
                .all(|owner| owner == Option::Some(Player::X))
        });

        let is_o_winner = lines.into_iter().any(|line| {
            line.into_iter()
                .map(|coord| self.board.get_at(coord))
                .all(|owner| owner == Option::Some(Player::O))
        });

        let is_board_full = self
            .board
            .clone()
            .into_iter()
            .all(|line| line.into_iter().all(|cell| cell != Option::None));
        let is_cats = is_board_full && !is_x_winner && !is_o_winner;

        if is_x_winner {
            Winner::Player(Player::X)
        } else if is_o_winner {
            Winner::Player(Player::O)
        } else if is_cats {
            Winner::CatsGame
        } else {
            Winner::NoWinner
        }
    }
}

impl Widget for &Winner {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let winner_text = match self {
            Winner::NoWinner => "",
            Winner::CatsGame => "Cat's Game",
            Winner::Player(Player::X) => "X Wins",
            Winner::Player(Player::O) => "O Wins",
        };

        let winner_block = Block::default()
            .borders(Borders::ALL)
            .title(Line::from(" Winner ").yellow())
            .border_type(BorderType::Rounded);

        Paragraph::new(winner_text)
            .centered()
            .bold()
            .block(winner_block)
            .render(area, buf);
    }
}

impl Widget for &GameState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = [
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ];
        let row_constraints = [
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ];
        let vertical = Layout::vertical(row_constraints);

        let row_rects = vertical.split(area);

        for (r, row_rect) in row_rects.iter().enumerate() {
            let col_rects = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(col_constraints.clone())
                .split(*row_rect);

            for (c, cell_rect) in col_rects.iter().enumerate() {
                let cell = self.cell((r, c));
                cell.render(*cell_rect, buf)
            }
        }
    }
}

impl<'game_state> Widget for &Cell<'game_state> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let current_player_text = format!(" {} ", self.game_state.current_player.to_string());

        let block = if self.is_active_cell() {
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(current_player_text)
        } else {
            Block::new().borders(Borders::NONE)
        };

        let text = match self.owner() {
            Option::Some(player) => format!("{}", player),
            Option::None => "_".to_string(),
        };

        Paragraph::new(text)
            .block(block)
            .centered()
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let game_state = GameState::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 40, 9));

        game_state.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "╭ X ───────╮      _           _         ",
            "│     _    │                            ",
            "╰──────────╯                            ",
            "      _           _           _         ",
            "                                        ",
            "      _           _           _         ",
            "                                        ",
            "                                        ",
            "                                        ",
        ]);

        let cell_style = Style::new();
        expected.set_style(Rect::new(0, 0, 0, 0), cell_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn game_play() -> color_eyre::Result<()> {
        let mut game_state = GameState::default();
        assert_eq!(game_state.current_player, Player::X);
        assert_eq!(game_state.board, Board::default());
        assert_eq!(game_state.winner(), Winner::NoWinner);

        game_state.handle_on_key_event(KeyCode::Enter.into());
        assert_eq!(game_state.current_player, Player::O);
        assert_eq!(
            game_state.board,
            [
                [Option::Some(Player::X), Option::None, Option::None],
                [Option::None, Option::None, Option::None],
                [Option::None, Option::None, Option::None],
            ]
        );

        game_state.handle_on_key_event(KeyCode::Char('l').into());
        game_state.handle_on_key_event(KeyCode::Enter.into());
        assert_eq!(
            game_state.board,
            [
                [
                    Option::Some(Player::X),
                    Option::Some(Player::O),
                    Option::None
                ],
                [Option::None, Option::None, Option::None],
                [Option::None, Option::None, Option::None],
            ]
        );

        game_state.handle_on_key_event(KeyCode::Char('j').into());
        game_state.handle_on_key_event(KeyCode::Enter.into());
        assert_eq!(
            game_state.board,
            [
                [
                    Option::Some(Player::X),
                    Option::Some(Player::O),
                    Option::None
                ],
                [Option::None, Option::Some(Player::X), Option::None],
                [Option::None, Option::None, Option::None],
            ]
        );
        game_state.handle_on_key_event(KeyCode::Char('h').into());
        game_state.handle_on_key_event(KeyCode::Enter.into());
        assert_eq!(
            game_state.board,
            [
                [
                    Option::Some(Player::X),
                    Option::Some(Player::O),
                    Option::None
                ],
                [
                    Option::Some(Player::O),
                    Option::Some(Player::X),
                    Option::None
                ],
                [Option::None, Option::None, Option::None],
            ]
        );
        game_state.handle_on_key_event(KeyCode::Char('j').into());
        game_state.handle_on_key_event(KeyCode::Char('l').into());
        game_state.handle_on_key_event(KeyCode::Char('l').into());
        game_state.handle_on_key_event(KeyCode::Enter.into());
        assert_eq!(
            game_state.board,
            [
                [
                    Option::Some(Player::X),
                    Option::Some(Player::O),
                    Option::None
                ],
                [
                    Option::Some(Player::O),
                    Option::Some(Player::X),
                    Option::None
                ],
                [Option::None, Option::None, Option::Some(Player::X)],
            ]
        );
        assert_eq!(game_state.winner(), Winner::Player(Player::X));

        Ok(())
    }

    #[test]
    fn cats_game() -> color_eyre::Result<()> {
        let mut game_state = GameState::new();
        game_state.board = [
            [
                Option::Some(Player::X),
                Option::Some(Player::O),
                Option::Some(Player::O),
            ],
            [
                Option::Some(Player::O),
                Option::Some(Player::X),
                Option::Some(Player::X),
            ],
            [
                Option::Some(Player::X),
                Option::Some(Player::X),
                Option::Some(Player::O),
            ],
        ];
        assert_eq!(game_state.winner(), Winner::CatsGame);

        Ok(())
    }
}
