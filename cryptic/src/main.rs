use std::rc::Rc;

use clap::Parser;
use indexmap::IndexMap;

mod knit;

use knit::{knit_knot, Knit};
use letter_boxed::trie::Trie;

const CORPUS: &str = include_str!("../data/unigram_freq.csv");

fn corpus() -> impl Iterator<Item = &'static str> {
    CORPUS.lines().map(|line| line.split_once(',').unwrap().0)
}

/*
const CORPUS: &str = include_str!("../data/words_alpha.txt");

fn corpus() -> impl Iterator<Item = &'static str> {
    CORPUS.lines()
}
*/

fn set(word: &str) -> u128 {
    let mut res = 0;
    for c in word.chars() {
        res += 1 << ((c as u64 % 32 - 1) * 4);
    }
    res
}

#[derive(Parser)]
enum Args {
    Case { start: String, end: String },
    Core { core: String },
    Knit,
    Hide { n: usize },
}

impl Args {
    fn run(self) {
        match self {
            Self::Case { start, end } => Self::case(start, end),
            Self::Core { core } => Self::core(core),
            Self::Knit => Self::knit(),
            Self::Hide { n } => Self::hide(n),
        }
    }

    fn case(start: String, end: String) {
        let mut interiors_of_matches: IndexMap<u128, Vec<String>> = IndexMap::new();
        let mut anagrams: IndexMap<u128, Vec<String>> = IndexMap::new();

        for line in corpus() {
            if let Some(interior) = line
                .strip_prefix(&start)
                .and_then(|line| line.strip_suffix(&end))
            {
                let interior_set = set(interior);
                interiors_of_matches
                    .entry(interior_set)
                    .or_default()
                    .push(line.to_owned());
                for a in anagrams.entry(interior_set).or_default() {
                    println!("{line:20} -> {a}")
                }
            }
            let line_set = set(line);
            anagrams.entry(line_set).or_default().push(line.to_owned());
            for m in interiors_of_matches.entry(line_set).or_default() {
                println!("{m:20} -> {line}")
            }
        }
    }

    fn core(core: String) {
        let core_set = set(&core);
        for line in corpus() {
            if line.len() > 1 && core_set == set(&line[1..line.len() - 1]) {
                println!("{}", line);
            }
        }
    }

    fn knit() {
        let trie = Trie::construct();
        knit_knot(&Knit::knew(&trie), &Knit::knew(&trie), &Knit::knew(&trie));
    }

    fn hide(n: usize) {
        let trie = Trie::construct();
        for hide in corpus().map(|word| Hide::word(&trie, word)) {
            for word in corpus().take(10000) {
                let (h, results) = hide.add(word);
                for r in results {
                    if r.len() >= n {
                        println!("{:30}{}", h.words.join(" "), r);
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct Hide<'a> {
    tries: Vec<Knit<'a>>,
    words: Vec<Rc<str>>,
}

impl<'a> Hide<'a> {
    fn word(trie: &'a Trie, word: &str) -> Self {
        let mut tries = vec![];
        for c in word.chars().skip(1) {
            tries.push(Knit::knew(trie));
            tries = tries.into_iter().filter_map(|t| t.push(c)).collect();
        }
        Self {
            tries,
            words: vec![Rc::from(word)],
        }
    }

    fn add(&self, word: &str) -> (Self, Vec<String>) {
        let mut results = vec![];
        let Self {
            mut tries,
            mut words,
        } = self.clone();
        for c in word.chars().take(word.len() - 1) {
            tries = tries
                .into_iter()
                .filter_map(|t| {
                    t.push(c).inspect(|t| {
                        if let Some(s) = t.try_get() {
                            results.push(s.to_owned())
                        }
                    })
                })
                .collect();
        }
        words.push(Rc::from(word));
        (Self { tries, words }, results)
    }
}

fn main() {
    Args::parse().run()
}
