use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    prelude::{Constraint, Direction, Layout, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use std::fmt;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}

type Coordinate = (usize, usize);

#[derive(Debug, Default, Clone)]
enum Player {
    #[default]
    X,
    O,
}
trait ToOwner {
    fn to_owner(&self) -> Owner;
}
impl ToOwner for Player {
    fn to_owner(&self) -> Owner {
        match self {
            Player::X => Owner::X,
            Player::O => Owner::O,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
enum Owner {
    #[default]
    N,
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

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

type Row = (Owner, Owner, Owner);
type Board = (Row, Row, Row);

trait Boardable {
    fn get_at(&self, coordinate: Coordinate) -> Owner;
    fn set_at(&self, coordinate: Coordinate, owner: Owner) -> Board;
}

impl Boardable for Board {
    fn get_at(&self, (row_idx, col_idx): Coordinate) -> Owner {
        match (row_idx, col_idx) {
            (0, 0) => self.0.0.clone(),
            (0, 1) => self.0.1.clone(),
            (0, 2) => self.0.2.clone(),
            (1, 0) => self.1.0.clone(),
            (1, 1) => self.1.1.clone(),
            (1, 2) => self.1.2.clone(),
            (2, 0) => self.2.0.clone(),
            (2, 1) => self.2.1.clone(),
            (2, 2) => self.2.2.clone(),
            (_, _) => self.0.0.clone(),
        }
    }

    fn set_at(&self, (row_idx, col_idx): Coordinate, owner: Owner) -> Board {
        let mut next: Board = self.clone();

        match (row_idx, col_idx) {
            (0, 0) => next.0.0 = owner,
            (0, 1) => next.0.1 = owner,
            (0, 2) => next.0.2 = owner,
            (1, 0) => next.1.0 = owner,
            (1, 1) => next.1.1 = owner,
            (1, 2) => next.1.2 = owner,
            (2, 0) => next.2.0 = owner,
            (2, 1) => next.2.1 = owner,
            (2, 2) => next.2.2 = owner,
            (_, _) => next.0.0 = owner,
        }

        next
    }
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Owner::X => write!(f, "X"),
            Owner::O => write!(f, "O"),
            Owner::N => write!(f, "_"),
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    game_state: GameState,
    exit: bool,
}

#[derive(Debug, Default)]
pub struct GameState {
    current_player: Player,
    board: Board,
    active_cell: Coordinate,
}

struct Cell<'game_state> {
    game_state: &'game_state GameState,
    row: usize,
    col: usize,
}

impl<'game_state> Cell<'game_state> {
    fn new(game_state: &'game_state GameState, row: usize, col: usize) -> Self {
        Self {
            game_state,
            row,
            col,
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    fn reset(&mut self) {
        self.board = Board::default()
    }

    fn cell(&self, (r, c): Coordinate) -> Cell<'_> {
        Cell::new(self, r, c)
    }

    fn move_active_cell(&mut self, direction: Dir) {
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

    fn make_move(&mut self) {
        if self.is_valid_move() {
            self.set_cell(self.active_cell.clone(), self.current_player.clone());
            self.toggle_current_player()
        }
    }

    fn is_valid_move(&mut self) -> bool {
        let is_current_cell_empty = self.board.get_at(self.active_cell) == Owner::N;
        let is_game_over = self.winner() != Owner::N;
        is_current_cell_empty && !is_game_over
    }

    fn set_cell(&mut self, active_cell: Coordinate, player: Player) {
        let owner = player.to_owner();
        let next_board = self.board.set_at(active_cell, owner);
        self.board = next_board;
    }

    fn toggle_current_player(&mut self) {
        match self.current_player {
            Player::O => self.current_player = Player::X,
            Player::X => self.current_player = Player::O,
        }
    }

    // TODO: Handle Cat's Game
    fn winner(&self) -> Owner {
        let lines = [
            [(0, 0), (0, 1), (0, 2)], // row 0
            [(1, 0), (1, 1), (1, 2)], // row 1
            [(2, 0), (2, 1), (2, 2)], // row 2
            [(0, 0), (1, 0), (2, 0)], // col 0
            [(0, 1), (1, 1), (2, 1)], // col 1
            [(0, 2), (1, 1), (2, 2)], // col 2
            [(0, 0), (1, 1), (2, 2)], // diag left to right
            [(0, 2), (1, 1), (2, 0)], // diag right to left
        ];

        let x_wins = lines.into_iter().any(|line| {
            line.into_iter()
                .map(|coord| self.board.get_at(coord))
                .all(|owner| owner == Owner::X)
        });

        let o_wins = lines.into_iter().any(|line| {
            line.into_iter()
                .map(|coord| self.board.get_at(coord))
                .all(|owner| owner == Owner::O)
        });

        if x_wins {
            Owner::X
        } else if o_wins {
            Owner::O
        } else {
            Owner::N
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let terminal_rect = frame.area();

        let instructions = Line::from(vec![
            " H,J,K,L ".into(),
            "<Move Cell>".blue().bold(),
            " Enter ".into(),
            "<Place>".blue().bold(),
            " R ".into(),
            "<Reset>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let title = Line::from(" Tic Tac Tui ").bold().yellow().centered();
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .title(title.centered())
            .title_bottom(instructions.clone().centered())
            .border_type(BorderType::Rounded);
        frame.render_widget(outer_block, terminal_rect);

        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .horizontal_margin(1)
            .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(terminal_rect);

        // Top
        let winner_text = match self.game_state.winner() {
            Owner::N => "",
            Owner::X => "X Wins",
            Owner::O => "O Wins",
        };

        let winner_block = Block::default()
            .borders(Borders::ALL)
            .title(Line::from(" Winner ").yellow())
            .border_type(BorderType::Rounded);

        let winner_paragraph = Paragraph::new(winner_text)
            .centered()
            .bold()
            .block(winner_block);

        frame.render_widget(winner_paragraph, outer_layout[0]);

        // Center
        frame.render_widget(self, outer_layout[1]);
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn handle_on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.exit(),
            (_, KeyCode::Char('h') | KeyCode::Left) => self.game_state.move_active_cell(Dir::Left),
            (_, KeyCode::Char('j') | KeyCode::Down) => self.game_state.move_active_cell(Dir::Down),
            (_, KeyCode::Char('k') | KeyCode::Up) => self.game_state.move_active_cell(Dir::Up),
            (_, KeyCode::Char('l') | KeyCode::Right) => {
                self.game_state.move_active_cell(Dir::Right)
            }
            (_, KeyCode::Enter) => self.game_state.make_move(),
            (_, KeyCode::Char('r')) => self.game_state.reset(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl<'game_state> Widget for &Cell<'game_state> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let owner: &Owner = &self.game_state.board.get_at((self.row, self.col));
        let active_cell = &self.game_state.active_cell;
        let current_player_text = format!(" {} ", self.game_state.current_player.to_string());
        let block;
        let is_active_cell = active_cell.0 == self.row && active_cell.1 == self.col;
        if is_active_cell {
            block = Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(current_player_text);
        } else {
            block = Block::new().borders(Borders::NONE);
        }

        let text = format!("{}", owner);

        Paragraph::new(text)
            .block(block)
            .centered()
            .render(area, buf);
    }
}

impl Widget for &App {
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
                let cell = self.game_state.cell((r, c));
                cell.render(*cell_rect, buf)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━━ Tic Tac Tui ━━━━━━━━━━━━━━━━━━┓",
            "┃                                                ┃",
            "┃                    |-|-|-|                     ┃",
            "┃                    |-|-|-|                     ┃",
            "┃                    |-|-|-|                     ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);

        let title_style = Style::new().blue().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 13, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> color_eyre::Result<()> {
        let mut app = App::default();
        app.handle_on_key_event(KeyCode::Right.into());
        // assert_eq!(app.counter, 1);

        app.handle_on_key_event(KeyCode::Left.into());
        // assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_on_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
