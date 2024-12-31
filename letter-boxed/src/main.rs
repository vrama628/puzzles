use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use layout::Flex;
use ratatui::{prelude::*, widgets::Block};

const CORPUS: &'static str = include_str!("../data/words_alpha.txt");
const CORPUS_LEN: usize = CORPUS.len();

#[derive(Default)]
struct Puzzle {
    letters: [Option<char>; 12],
    cursor: usize,
}

impl Puzzle {
    const WIDTH: u16 = 15;

    fn insert(&mut self, c: char) {
        match self.letters[self.cursor].replace(c.to_ascii_uppercase()) {
            Some(_) => self.cursor = self.next_cursor_position(),
            None => self.cursor = self.next_empty_cursor_position(),
        }
    }

    fn delete(&mut self) {
        if self.letters[self.cursor].take().is_some() {
            self.cursor = self.previous_cursor_position();
        }
    }

    fn next_empty_cursor_position(&self) -> usize {
        self.letters
            .iter()
            .enumerate()
            .skip(self.cursor)
            .find(|(_, l)| l.is_none())
            .map(|(i, _)| i)
            .unwrap_or_else(|| self.next_cursor_position())
    }

    fn next_cursor_position(&self) -> usize {
        (self.cursor + 1) % 12
    }

    fn previous_cursor_position(&self) -> usize {
        (self.cursor + 11) % 12
    }

    fn move_cursor_forward(&mut self) {
        self.cursor = self.next_cursor_position();
    }

    fn move_cursor_backward(&mut self) {
        self.cursor = self.previous_cursor_position();
    }
}

impl Widget for &Puzzle {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [top, horizontal, bottom] = Layout::vertical([1, 7, 1]).areas(area);
        let [left, block, right] = Layout::horizontal([1, 13, 1]).areas(horizontal);
        let horizontal_letters = Layout::horizontal([1, 1, 1])
            .spacing(4)
            .horizontal_margin(2);
        let vertical_letters = Layout::vertical([1, 1, 1]).spacing(1).vertical_margin(1);
        let top = horizontal_letters.split(top);
        let right = vertical_letters.split(right);
        let bottom = horizontal_letters.split(bottom);
        let left = vertical_letters.split(left);
        for (i, rect) in top
            .into_iter()
            .chain(right.into_iter())
            .chain(bottom.into_iter().rev())
            .chain(left.into_iter().rev())
            .enumerate()
        {
            let content = self.letters[i].unwrap_or(' ').to_string();
            let style = if i == self.cursor {
                Style::default().fg(Color::White).bg(Color::Black)
            } else {
                Style::default().fg(Color::Black).bg(Color::White)
            };
            Span::styled(content, style).render(*rect, buf);
        }
        Block::bordered().render(block, buf);
    }
}

// trie to deal with adjacencies
// then process into map from first to last letter with bitvector of letters used
// progress steps: processing corpus, checking for one word solutions, two word solutions, etc.

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();
    let mut puzzle = Puzzle::default();
    loop {
        terminal.draw(|f| {
            let [area] = Layout::horizontal([Puzzle::WIDTH])
                .flex(Flex::Center)
                .vertical_margin(2)
                .areas(f.area());
            f.render_widget(&puzzle, area)
        })?;
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                puzzle.insert(c);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => puzzle.delete(),
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                puzzle.move_cursor_forward();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                puzzle.move_cursor_backward();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                todo!();
            }
            _ => {}
        }
    }
    ratatui::restore();
    Ok(())
}
