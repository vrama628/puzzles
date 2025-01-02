use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use layout::Flex;
use ratatui::{prelude::*, DefaultTerminal};

mod puzzle;
use puzzle::Puzzle;
mod solver;
use solver::Solver;
mod trie;

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
        // TODO: it would be nifty to move these into a method on Puzzle
        // that returns a ControlFlow
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
    terminal.draw(|f| {
        let [puzzle_area, solving_area] = layout(f.area());
        f.render_widget(&puzzle, puzzle_area);
        f.render_widget(Span::raw("Solving..."), solving_area);
    })?;
    let solver = Solver::new(letters);
    let mut result = solver.solve();
    loop {
        terminal.draw(|f| {
            let [puzzle_area, solving_area] = layout(f.area());
            f.render_widget(&puzzle, puzzle_area);
            f.render_widget(&result, solving_area);
        })?;
        match read_key()? {
            KeyCode::Esc => return Ok(()),
            KeyCode::Down => result.scroll_down(),
            KeyCode::Up => result.scroll_up(),
            _ => {}
        }
    }
}
