use std::{iter::Peekable, str::Lines};

const CORPUS: &'static str = include_str!("../data/words_alpha.txt");

pub struct Trie {
    pub contains: bool,
    pub children: [Option<Box<Trie>>; 26],
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
    pub fn construct() -> Self {
        let mut lines = CORPUS.lines().peekable();
        Self::construct_from("", &mut lines)
    }

    pub fn take(&self, prefix: &str, mut n: usize) -> Vec<String> {
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

    pub fn filter(&self, prefix: &str) -> Option<&Self> {
        if prefix.is_empty() {
            Some(self)
        } else {
            let position = prefix.as_bytes()[0] % 32 - 1;
            let next = self.children[position as usize].as_ref()?;
            next.filter(&prefix[1..])
        }
    }

    pub fn filter_and_take(&self, prefix: &str, n: usize) -> Vec<String> {
        let Some(suffix) = self.filter(prefix) else {
            return vec![];
        };
        suffix.take(prefix, n)
    }
}
