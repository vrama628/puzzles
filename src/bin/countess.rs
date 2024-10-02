use std::{
    collections::{HashSet, VecDeque},
    vec,
};

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Tile {
    Gold,
    GoldUp,
    GoldDown,
    GoldRight,
    GoldLeft,
    Silver,
}

impl Tile {
    fn r#move(&self, direction: &Direction) -> Self {
        match (self, direction) {
            (Self::Gold, Direction::Up) => Self::GoldUp,
            (Self::Gold, Direction::Down) => Self::GoldDown,
            (Self::Gold, Direction::Right) => Self::GoldRight,
            (Self::Gold, Direction::Left) => Self::GoldLeft,
            (Self::GoldUp, Direction::Up) => Self::Silver,
            (Self::GoldUp, Direction::Down) => Self::Gold,
            (Self::GoldUp, Direction::Right) => Self::GoldUp,
            (Self::GoldUp, Direction::Left) => Self::GoldUp,
            (Self::GoldDown, Direction::Up) => Self::Gold,
            (Self::GoldDown, Direction::Down) => Self::Silver,
            (Self::GoldDown, Direction::Right) => Self::GoldDown,
            (Self::GoldDown, Direction::Left) => Self::GoldDown,
            (Self::GoldRight, Direction::Up) => Self::GoldRight,
            (Self::GoldRight, Direction::Down) => Self::GoldRight,
            (Self::GoldRight, Direction::Right) => Self::Silver,
            (Self::GoldRight, Direction::Left) => Self::Gold,
            (Self::GoldLeft, Direction::Up) => Self::GoldLeft,
            (Self::GoldLeft, Direction::Down) => Self::GoldLeft,
            (Self::GoldLeft, Direction::Right) => Self::Gold,
            (Self::GoldLeft, Direction::Left) => Self::Silver,
            (Self::Silver, Direction::Up) => Self::GoldDown,
            (Self::Silver, Direction::Down) => Self::GoldUp,
            (Self::Silver, Direction::Right) => Self::GoldLeft,
            (Self::Silver, Direction::Left) => Self::GoldRight,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State(Vec<Vec<Option<Tile>>>);

impl State {
    fn moves_from(&self, r: usize, c: usize) -> Vec<(Direction, Self)> {
        let mut res = vec![];
        if self.0[r][c].is_none() {
            return res;
        }
        if r > 0 && self.0[r - 1][c].is_none() {
            let mut new = self.clone();
            new.0[r - 1][c] = Some(new.0[r][c].as_ref().unwrap().clone().r#move(&Direction::Up));
            new.0[r][c] = None;
            res.push((Direction::Up, new))
        }
        if r < 2 && self.0[r + 1][c].is_none() {
            let mut new = self.clone();
            new.0[r + 1][c] = Some(
                new.0[r][c]
                    .as_ref()
                    .unwrap()
                    .clone()
                    .r#move(&Direction::Down),
            );
            new.0[r][c] = None;
            res.push((Direction::Down, new))
        }
        if c > 0 && self.0[r][c - 1].is_none() {
            let mut new = self.clone();
            new.0[r][c - 1] = Some(
                new.0[r][c]
                    .as_ref()
                    .unwrap()
                    .clone()
                    .r#move(&Direction::Left),
            );
            new.0[r][c] = None;
            res.push((Direction::Left, new))
        }
        if c < 2 && self.0[r][c + 1].is_none() {
            let mut new = self.clone();
            new.0[r][c + 1] = Some(
                new.0[r][c]
                    .as_ref()
                    .unwrap()
                    .clone()
                    .r#move(&Direction::Right),
            );
            new.0[r][c] = None;
            res.push((Direction::Right, new))
        }
        res
    }

    fn moves(&self) -> Vec<(Direction, Self)> {
        (0..3)
            .flat_map(|r| (0..3).flat_map(move |c| self.moves_from(r, c)))
            .collect()
    }

    fn start() -> Self {
        Self(vec![
            vec![Some(Tile::Gold), Some(Tile::Gold), Some(Tile::Gold)],
            vec![Some(Tile::Gold), None, Some(Tile::Gold)],
            vec![Some(Tile::Gold), Some(Tile::Gold), Some(Tile::Gold)],
        ])
    }

    fn is_done(&self) -> bool {
        self.0.iter().all(|row| {
            row.iter()
                .all(|cell| matches!(cell, None | Some(Tile::Silver)))
        })
    }
}

fn main() {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(State::start());
    queue.push_back((State::start(), vec![]));
    while let Some((state, moves)) = queue.pop_front() {
        if state.is_done() {
            return println!("{moves:#?}");
        }
        for (direction, new_state) in state.moves() {
            if visited.contains(&new_state) {
                continue;
            }
            let mut new_moves = moves.clone();
            new_moves.push(direction);
            visited.insert(new_state.clone());
            queue.push_back((new_state, new_moves));
        }
    }
}
