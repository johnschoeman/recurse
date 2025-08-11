use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    prelude::{Constraint, Direction, Layout, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Widget},
};

mod game_state;
use crate::app::game_state::GameState;

#[derive(Debug, Default)]
pub struct App {
    game_state: GameState,
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
        frame.render_widget(self, frame.area());
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
            _ => self.game_state.handle_on_key_event(key),
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        fn render_outer(area: Rect, buf: &mut Buffer) {
            let title = Line::from(" Tic Tac Tui ").bold().yellow().centered();

            let instructions = Line::from(vec![
                " Move ".into(),
                "<H,J,K,L>".blue().bold(),
                " Place ".into(),
                "<Enter>".blue().bold(),
                " Reset ".into(),
                "<R>".blue().bold(),
                " Quit ".into(),
                "<Q> ".blue().bold(),
            ]);

            Block::default()
                .borders(Borders::ALL)
                .title(title.centered())
                .title_bottom(instructions.clone().centered())
                .border_type(BorderType::Rounded)
                .render(area, buf);
        }

        render_outer(area, buf);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(1)
            .horizontal_margin(1)
            .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(area);

        self.game_state.winner().render(layout[0], buf);

        self.game_state.render(layout[1], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 53, 3));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "╭─────────────────── Tic Tac Tui ───────────────────╮",
            "│       _               _              _            │",
            "╰─ Move <H,J,K,L> Place <Enter> Reset <R> Quit <Q> ─╯",
        ]);

        let title_style = Style::new().yellow().bold();
        let instruction_style = Style::new().blue().bold();
        expected.set_style(Rect::new(20, 0, 13, 1), title_style);
        expected.set_style(Rect::new(8, 2, 9, 1), instruction_style);
        expected.set_style(Rect::new(24, 2, 7, 1), instruction_style);
        expected.set_style(Rect::new(38, 2, 3, 1), instruction_style);
        expected.set_style(Rect::new(47, 2, 4, 1), instruction_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> color_eyre::Result<()> {
        let mut app = App::default();
        app.handle_on_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
