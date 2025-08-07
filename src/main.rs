use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style},
    style::{Modifier, Stylize},
    symbols::border,
    text::{Line, Span},
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

#[derive(Debug, Default)]
enum RowIdx {
    #[default]
    R0,
    R1,
    R2,
}

#[derive(Debug, Default)]
enum ColIdx {
    #[default]
    C0,
    C1,
    C2,
}

type Coordinate = (usize, usize);

#[derive(Debug, Default, Clone)]
enum Player {
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

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Default)]
enum Col {
    #[default]
    N,
    X,
    O,
}
type Row = (Col, Col, Col);
type Board = (Row, Row, Row);

struct Ui {}

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

// #[derive(Debug, Default)]
// struct Cell {
//     app
//     owner: Option<Player>,
//     is_active: bool,
// }

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

    pub fn cell(&self, (r, c): Coordinate) -> Cell {
        Cell::new(self, r, c)
    }

    fn move_active_cell(&mut self, direction: Dir) {
        match direction {
            Dir::Left => {
                if (self.active_cell.1 > 0) {
                    let next_active_cell = (self.active_cell.0, self.active_cell.1 - 1);
                    self.active_cell = next_active_cell;
                }
            }
            Dir::Right => {
                if (self.active_cell.1 < 2) {
                    let next_active_cell = (self.active_cell.0, self.active_cell.1 + 1);
                    self.active_cell = next_active_cell;
                }
            }
            Dir::Up => {
                if (self.active_cell.0 > 0) {
                    let next_active_cell = (self.active_cell.0 - 1, self.active_cell.1);
                    self.active_cell = next_active_cell;
                }
            }
            Dir::Down => {
                if (self.active_cell.0 < 2) {
                    let next_active_cell = (self.active_cell.0 + 1, self.active_cell.1);
                    self.active_cell = next_active_cell;
                }
            }
        }
    }

    fn toggle_current_player(&mut self) {
        match self.current_player {
            Player::O => self.current_player = Player::X,
            Player::X => self.current_player = Player::O,
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

        let outer_block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                "Tic Tac Tui",
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            ))
            .border_type(BorderType::Rounded);
        frame.render_widget(outer_block, terminal_rect);

        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .horizontal_margin(1)
            .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(terminal_rect);

        // frame.render_widget(self, frame.area());
        frame.render_widget(self, outer_layout[0]);

        let current_player_text = self.game_state.current_player.to_string();
        let active_cell_row_text = self.game_state.active_cell.0.to_string();
        let active_cell_col_text = self.game_state.active_cell.1.to_string();
        frame.render_widget(
            Paragraph::new(Line::from_iter([
                "Current Player: ".into(),
                current_player_text,
                " Active Cell: ".into(),
                active_cell_row_text,
                " ".into(),
                active_cell_col_text,
            ]))
            .block(Block::new().borders(Borders::ALL)),
            outer_layout[1],
        );
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
            (_, KeyCode::Left) => self.game_state.move_active_cell(Dir::Left),
            (_, KeyCode::Right) => self.game_state.move_active_cell(Dir::Right),
            (_, KeyCode::Up) => self.game_state.move_active_cell(Dir::Up),
            (_, KeyCode::Down) => self.game_state.move_active_cell(Dir::Down),
            (_, KeyCode::Enter) => self.game_state.toggle_current_player(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

// impl Widget for &Cell<' _> {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         let text = show_player(self.owner.clone());
//         let block;
//         if self.is_active {
//             block = Block::bordered().border_set(border::THICK);
//         } else {
//             block = Block::bordered().border_set(border::PLAIN);
//         };
//         Paragraph::new(text)
//             .block(block)
//             .centered()
//             .render(area, buf);
//     }
// }

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Tic Tac Tui ").bold().blue().centered();
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        // Paragraph::new(game_text)
        //     .centered()
        //     .block(block)
        //     .render(area, buf);

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
        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints);

        let row_rects = vertical.split(area);

        for (r, row_rect) in row_rects.iter().enumerate() {
            let col_rects = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(0)
                .horizontal_margin(1)
                .constraints(col_constraints.clone())
                .split(*row_rect);

            for (c, cell_rect) in col_rects.iter().enumerate() {
                let cell = self.game_state.cell((r, c));

                // c.render(cell, buf)
                // Paragraph::new(format!("Area {:02}", i + 1))
                // .block(Block::bordered())
                // .render(cell, buf);
            }
        }
    }
}

// fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
//     let [area] = Layout::horizontal([horizontal])
//         .flex(Flex::Center)
//         .areas(area);
//     let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
//     area
// }

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
