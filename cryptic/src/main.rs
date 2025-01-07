use clap::Parser;
use hide::Hide;
use indexmap::IndexMap;

mod hide;
mod knit;

use knit::{knit_knot, Knit};
use letter_boxed::trie::Trie;

const FREQ_CORPUS: &str = include_str!("../data/unigram_freq.csv");
const SCOWL_CORPUS: &str = include_str!("../data/scowl/final/english-words.35");
const SIMPLE_CORPUS: &str = include_str!("../data/words_alpha.txt");

fn scowl_corpus() -> impl Iterator<Item = &'static str> {
    SCOWL_CORPUS
        .lines()
        .filter(|w| w.chars().all(|c| c.is_alphabetic()))
}

fn freq_corpus() -> impl Iterator<Item = &'static str> {
    FREQ_CORPUS
        .lines()
        .map(|line| line.split_once(',').unwrap().0)
}

fn simple_corpus() -> impl Iterator<Item = &'static str> {
    SIMPLE_CORPUS.lines()
}

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
    Hide { target: String },
}

impl Args {
    fn run(self) {
        match self {
            Self::Case { start, end } => Self::case(start, end),
            Self::Core { core } => Self::core(core),
            Self::Knit => Self::knit(),
            Self::Hide { target } => Self::hide(target),
        }
    }

    fn case(start: String, end: String) {
        let mut interiors_of_matches: IndexMap<u128, Vec<String>> = IndexMap::new();
        let mut anagrams: IndexMap<u128, Vec<String>> = IndexMap::new();

        for line in freq_corpus() {
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
        for line in freq_corpus() {
            if line.len() > 1 && core_set == set(&line[1..line.len() - 1]) {
                println!("{}", line);
            }
        }
    }

    fn knit() {
        let trie = Trie::construct();
        knit_knot(&Knit::knew(&trie), &Knit::knew(&trie), &Knit::knew(&trie));
    }

    fn hide(target: String) {
        let mut hide = Hide::new(target);
        for word in scowl_corpus() {
            hide.add(word);
        }
    }
}

fn main() {
    Args::parse().run()
}
