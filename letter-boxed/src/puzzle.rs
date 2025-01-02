use ratatui::{prelude::*, widgets::Block};

#[derive(Default)]
pub struct Puzzle {
    letters: [Option<char>; 12],
    cursor: usize,
}

impl Puzzle {
    pub const WIDTH: u16 = 15;
    pub const HEIGHT: u16 = 9;

    pub fn insert(&mut self, c: char) {
        match self.letters[self.cursor].replace(c.to_ascii_uppercase()) {
            Some(_) => self.cursor = self.next_cursor_position(),
            None => self.cursor = self.next_empty_cursor_position(),
        }
    }

    pub fn delete(&mut self) {
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

    pub fn move_cursor_forward(&mut self) {
        self.cursor = self.next_cursor_position();
    }

    pub fn move_cursor_backward(&mut self) {
        self.cursor = self.previous_cursor_position();
    }

    pub fn is_filled(&self) -> Option<[char; 12]> {
        if self.letters.iter().all(|l| l.is_some()) {
            Some(self.letters.map(|l| l.unwrap()))
        } else {
            None
        }
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
