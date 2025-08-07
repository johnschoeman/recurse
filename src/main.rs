use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    prelude::{Constraint, Direction, Layout},
    DefaultTerminal, Frame,
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
enum Row {
    #[default] R0,
    R1,
    R2,
}

#[derive(Debug, Default)]
enum Col {
    #[default] C0,
    C1,
    C2,
}

type CellPos = ( Row, Col );

#[derive(Debug, Default, Clone)]
enum Player {
    #[default] X,
    O,
}
fn show_player(player: Option<Player>) -> String {
    "X".to_string()
}

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Default)]
pub struct App {
    current_player: Player,
    board: Vec<Vec<Cell>>,
    active_cell: CellPos,
    exit: bool,
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
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ])
            .split(frame.area());



        // frame.render_widget(self, frame.area());
        frame.render_widget(self, outer_layout[0]);

        frame.render_widget(
            Paragraph::new("Bottom")
                .block(Block::new().borders(Borders::ALL)),
            outer_layout[1]);
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
            (_, KeyCode::Left) => self.move_active_cell(Dir::Left),
            (_, KeyCode::Right) => self.move_active_cell(Dir::Right),
            (_, KeyCode::Up) => self.move_active_cell(Dir::Up),
            (_, KeyCode::Down) => self.move_active_cell(Dir::Down),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn move_active_cell(&mut self, direction: Dir) {
        let next_active_cell: CellPos = (Row::R1, Col::C2);
        self.active_cell = next_active_cell;
    }
}


#[derive(Debug, Default)]
struct Cell {
    owner: Option<Player>,
    is_active: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget for &Cell {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = show_player(self.owner.clone());

        let block;
        if self.is_active {
            block = Block::bordered().border_set(border::THICK);
        } else {
            block = Block::bordered().border_set(border::PLAIN);
        };

        Paragraph::new(text)
            .block(block)
            .centered()
            .render(area, buf);
    }
}


impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Tic Tac Tui ")
            .bold()
            .blue()
            .centered();
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

        let col_constraints = (0..3).map(|_| Constraint::Percentage(30));
        let row_constraints = (0..3).map(|_| Constraint::Percentage(30));
        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            let c = Cell::new();
            c.render(cell, buf)
            // Paragraph::new(format!("Area {:02}", i + 1))
                // .block(Block::bordered())
                // .render(cell, buf);
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
