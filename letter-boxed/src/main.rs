use std::{fmt::Display, iter::Peekable, str::Lines};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use layout::Flex;
use ratatui::{
    prelude::*,
    widgets::{Block, List},
    DefaultTerminal,
};

const CORPUS: &'static str = include_str!("../data/words_alpha.txt");
const CORPUS_LEN: usize = CORPUS.len();

struct Trie {
    contains: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn construct_from(prefix: &str, lines: &mut Peekable<Lines>) -> Self {
        let contains = lines.next_if_eq(&prefix).is_some();
        let mut children = [const { None }; 26];
        while lines.peek().is_some_and(|&word| word.starts_with(prefix)) {
            let next_prefix = &lines.peek().unwrap()[..=prefix.len()];
            let position = next_prefix.as_bytes()[prefix.len()] % 32 - 1;
            children[position as usize] = Some(Box::new(Self::construct_from(next_prefix, lines)));
        }
        Self { contains, children }
    }

    /// assumes corpus is sorted alphabetically and contains no duplicates
    fn construct() -> Self {
        let mut lines = CORPUS.lines().peekable();
        Self::construct_from("", &mut lines)
    }

    fn take(&self, prefix: &str, mut n: usize) -> Vec<String> {
        let mut results = Vec::with_capacity(n);
        if self.contains {
            results.push(prefix.to_owned());
            n -= 1;
        }
        for (i, child) in self
            .children
            .iter()
            .enumerate()
            .filter_map(|(i, child)| child.as_ref().map(|c| (i, c)))
        {
            if n == 0 {
                break;
            }
            let c = (i as u8 + 'A' as u8) as char;
            let next_prefix = format!("{prefix}{c}");
            let next_results = child.take(&next_prefix, n);
            n -= next_results.len();
            results.extend(next_results);
        }
        results
    }

    fn filter(&self, prefix: &str) -> Option<&Self> {
        if prefix.is_empty() {
            Some(self)
        } else {
            let position = prefix.as_bytes()[0] % 32 - 1;
            let next = self.children[position as usize].as_ref()?;
            next.filter(&prefix[1..])
        }
    }

    fn filter_and_take(&self, prefix: &str, n: usize) -> Vec<String> {
        let Some(suffix) = self.filter(prefix) else {
            return vec![];
        };
        suffix.take(prefix, n)
    }
}

#[derive(Default)]
struct Puzzle {
    letters: [Option<char>; 12],
    cursor: usize,
}

impl Puzzle {
    const WIDTH: u16 = 15;
    const HEIGHT: u16 = 9;

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

    fn is_filled(&self) -> Option<[char; 12]> {
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

struct Input {
    trie: Trie,
    input: String,
}

impl Input {
    fn new() -> Self {
        Self {
            trie: Trie::construct(),
            input: String::new(),
        }
    }

    fn insert(&mut self, c: char) {
        self.input.push(c.to_ascii_uppercase());
    }

    fn delete(&mut self) {
        self.input.pop();
    }
}

impl Widget for &Input {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [search_area, results_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
                .spacing(1)
                .areas(area);
        Line::from_iter([Span::raw(&self.input), Span::raw(" ").underlined()])
            .render(search_area, buf);
        let n_results = results_area.height;
        let results = self.trie.filter_and_take(&self.input, n_results as usize);
        Widget::render(List::new(results), results_area, buf);
    }
}

// trie to deal with adjacencies
// then process into map from first to last letter with bitvector of letters used
// progress steps: processing corpus, checking for one word solutions, two word solutions, etc.

fn layout(area: Rect) -> [Rect; 2] {
    let [puzzle_area, solving_area] =
        Layout::vertical([Constraint::Length(Puzzle::HEIGHT), Constraint::Fill(1)])
            .margin(2)
            .spacing(2)
            .areas(area);
    let [puzzle_area] = Layout::horizontal([Puzzle::WIDTH])
        .flex(Flex::Center)
        .areas(puzzle_area);
    [puzzle_area, solving_area]
}

fn read_key() -> std::io::Result<KeyCode> {
    match event::read()? {
        Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: _,
        }) => Ok(code),
        _ => read_key(),
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}

fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut puzzle = Puzzle::default();
    let letters = loop {
        terminal.draw(|f| {
            let [puzzle_area, solving_area] = layout(f.area());
            f.render_widget(&puzzle, puzzle_area);
            if puzzle.is_filled().is_some() {
                f.render_widget(Span::raw("Press enter to start solving"), solving_area)
            }
        })?;
        match read_key()? {
            KeyCode::Char(c) => puzzle.insert(c),
            KeyCode::Backspace => puzzle.delete(),
            KeyCode::Right => puzzle.move_cursor_forward(),
            KeyCode::Left => puzzle.move_cursor_backward(),
            KeyCode::Esc => return Ok(()),
            KeyCode::Enter => {
                if let Some(letters) = puzzle.is_filled() {
                    break letters;
                }
            }
            _ => {}
        }
    };
    let mut input = Input::new();
    loop {
        terminal.draw(|f| {
            let [puzzle_area, solving_area] = layout(f.area());
            f.render_widget(&puzzle, puzzle_area);
            f.render_widget(&input, solving_area);
        })?;
        match read_key()? {
            KeyCode::Char(c) => input.insert(c),
            KeyCode::Backspace => input.delete(),
            KeyCode::Esc => return Ok(()),
            _ => {}
        }
    }
}
