use std::{cmp::Ordering, collections::BinaryHeap};

use ratatui::prelude::*;

use crate::trie::Trie;

#[derive(PartialEq, Eq, Clone)]
struct Solution {
    used: u32,
    words: Vec<String>,
}

impl Solution {
    fn last_char(&self) -> char {
        *self.words.last().unwrap().as_bytes().last().unwrap() as char
    }

    fn add(&self, other: &Self) -> Self {
        let used = self.used | other.used;
        let mut words = self.words.clone();
        words.extend(other.words.clone());
        Self { used, words }
    }
}

impl Ord for Solution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.used
            .count_ones()
            .cmp(&other.used.count_ones())
            .then(self.used.cmp(&other.used))
            .then(self.words.cmp(&other.words))
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn set(c: char) -> u32 {
    1 << (c as u32 % 32 - 1)
}

pub struct Solver {
    trie: Trie,
    letters: [char; 12],
    needed: u32,
}

impl Solver {
    pub fn new(letters: [char; 12]) -> Self {
        let mut needed = 0;
        for c in letters {
            needed |= set(c);
        }
        Self {
            trie: Trie::construct(),
            letters,
            needed,
        }
    }

    fn index_of(&self, c: char) -> usize {
        self.letters.iter().position(|&l| l == c).unwrap()
    }

    /// precondition: `used` represents the letters in the prefix
    fn collect_solutions_from_trie(
        &self,
        trie: &Trie,
        prefix: &str,
        used: u32,
        results: &mut Vec<Solution>,
    ) {
        if trie.contains {
            results.push(Solution {
                used,
                words: vec![prefix.to_owned()],
            });
        }
        let x = *prefix.as_bytes().last().unwrap();
        let from_side = self.index_of(x as char) / 3;
        for i in 0..12 {
            if i / 3 == from_side {
                continue;
            }
            let c = self.letters[i];
            if let Some(next) = trie.children[c as usize % 32 - 1].as_ref() {
                let next_prefix = format!("{prefix}{c}");
                let next_used = used | set(c);
                self.collect_solutions_from_trie(next, &next_prefix, next_used, results);
            }
        }
    }

    pub fn solve(&self) -> Result {
        let words = self.letters.map(|c| {
            let mut results = vec![];
            if let Some(trie) = self.trie.children[c as usize % 32 - 1].as_ref() {
                self.collect_solutions_from_trie(trie, &c.to_string(), set(c), &mut results);
            }
            results
        });
        let mut solutions: BinaryHeap<Solution> = words.clone().into_iter().flatten().collect();
        let mut result = Result::default();
        loop {
            result.counts.push(solutions.len());
            if solutions
                .peek()
                .is_some_and(|solution| solution.used == self.needed)
            {
                while let Some(solution) = solutions.pop() {
                    if solution.used == self.needed {
                        result.results.push(solution.words);
                    } else {
                        break;
                    }
                }
                result.results.reverse();
                return result;
            }
            solutions = solutions
                .into_iter()
                .flat_map(|solution| {
                    let relevant_words = &words[self.index_of(solution.last_char())];
                    relevant_words.iter().map(move |word| solution.add(word))
                })
                .collect();
        }
    }
}

#[derive(Default)]
pub struct Result {
    scroll: usize,
    counts: Vec<usize>,
    results: Vec<Vec<String>>,
}

impl Result {
    pub fn scroll_down(&mut self) {
        self.scroll = (self.scroll + 1).min(self.results.len() - 1);
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }
}

impl Widget for &Result {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let n_words = self.counts.len();
        let mut text = Text::default();
        for (i, count) in self.counts.iter().enumerate() {
            let mut line = format!("Considered {count} permutations of length {}...", i + 1);
            if i != n_words - 1 {
                line.push_str(" and found no solutions.");
            }
            text.push_line(line);
        }
        text.push_line(format!(
            "Found {} solutions of length {n_words}:",
            self.results.len()
        ));
        for result in self.results[self.scroll..]
            .iter()
            .take(area.height as usize - text.height())
        {
            text.push_line(result.join("-"))
        }
        text.render(area, buf);
    }
}
