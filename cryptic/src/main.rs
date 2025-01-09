use std::collections::HashSet;

use clap::Parser;
use hide::Hide;
use indexmap::{IndexMap, IndexSet};

mod hide;
mod knit;

use knit::{knit_knot, Knit};
use letter_boxed::trie::Trie;

const FREQ_CORPUS: &str = include_str!("../data/unigram_freq.csv");
const SCOWL_CORPUS_10: &str = include_str!("../data/scowl/final/english-words.10");
const SCOWL_CORPUS_20: &str = include_str!("../data/scowl/final/english-words.20");
const SCOWL_CORPUS_35: &str = include_str!("../data/scowl/final/english-words.35");
const SIMPLE_CORPUS: &str = include_str!("../data/words_alpha.txt");

fn scowl_corpus() -> impl Iterator<Item = &'static str> {
    SCOWL_CORPUS_10
        .lines()
        .chain(SCOWL_CORPUS_20.lines())
        .chain(SCOWL_CORPUS_35.lines())
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
    Case {
        start: String,
        end: String,
    },
    Core {
        core: String,
    },
    Knit,
    Hide {
        #[arg(short, long)]
        minimum_overlap: Option<usize>,
        target: String,
    },
    Ruin {
        target: Vec<String>,
    },
    Tuck {
        character: Option<char>,
    },
}

impl Args {
    fn run(self) {
        match self {
            Self::Case { start, end } => Self::case(start, end),
            Self::Core { core } => Self::core(core),
            Self::Knit => Self::knit(),
            Self::Hide {
                target,
                minimum_overlap,
            } => Self::hide(target, minimum_overlap),
            Self::Ruin { target } => Self::ruin(target),
            Self::Tuck { character } => Self::tuck(character),
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

    fn hide(target: String, minimum_overlap: Option<usize>) {
        let mut hide = Hide::new(target, minimum_overlap);
        for word in scowl_corpus() {
            hide.add(word);
        }
    }

    fn ruin(target: Vec<String>) {
        let mut target_set = 0;
        for word in target {
            target_set += set(&word);
        }
        let words: Vec<(&str, u128)> = scowl_corpus().map(|word| (word, set(word))).collect();
        let mut candidates: Vec<(Vec<&str>, u128)> = vec![(vec![], 0)];
        while !candidates.is_empty() {
            candidates = candidates
                .into_iter()
                .flat_map(|(candidate_words, candidate_set)| {
                    words.iter().filter_map(move |(word, word_set)| {
                        let new_set = candidate_set + word_set;
                        (candidate_words.last().is_none_or(|&last| last <= word)
                            && (0..32).all(|i| {
                                new_set & (0b1111 << 4 * i) <= target_set & (0b1111 << 4 * i)
                            }))
                        .then(|| {
                            let mut words = candidate_words.clone();
                            words.push(word);
                            (words, new_set)
                        })
                    })
                })
                .inspect(|(words, set)| {
                    if *set == target_set {
                        println!("{}", words.join(" "))
                    }
                })
                .collect();
        }
    }

    fn tuck(character: Option<char>) {
        let mut one_removed: IndexMap<String, Vec<(&str, char)>> = IndexMap::new();
        let mut words: HashSet<&str> = HashSet::new();
        for word in scowl_corpus() {
            words.insert(word);
            for (i, c) in word
                .char_indices()
                .take(word.len() - 1)
                .skip(1)
                .filter(|&(_, c)| character.is_none_or(|ch| c == ch))
            {
                let removed = format!("{}{}", &word[..i], &word[i + 1..]);
                if words.contains(removed.as_str()) {
                    println!("{removed:15}+ {c} = {word}")
                }
                one_removed.entry(removed).or_default().push((word, c));
            }
            // look up all words that can one letter can be removed from to get this
            if let Some(removeds) = one_removed.get(word) {
                for (removed, c) in removeds {
                    println!("{word:15}+ {c} = {removed}")
                }
            }
        }
    }
}

fn main() {
    Args::parse().run()
}
