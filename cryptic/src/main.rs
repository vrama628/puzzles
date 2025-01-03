use std::collections::BTreeMap;

use clap::Parser;

const CORPUS: &str = include_str!("../data/words_alpha.txt");

fn set(word: &str) -> u128 {
    let mut res = 0;
    for c in word.chars() {
        res += 1 << ((c as u64 % 32 - 1) * 4);
    }
    res
}

#[derive(Parser)]
enum Args {
    Case { start: char, end: char },
    Core { core: String },
}

impl Args {
    fn run(self) {
        match self {
            Self::Case { start, end } => Self::case(start, end),
            Self::Core { core } => Self::core(core),
        }
    }

    fn case(start: char, end: char) {
        let mut interiors_of_matches: BTreeMap<u128, Vec<String>> = BTreeMap::new();
        let mut anagrams: BTreeMap<u128, Vec<String>> = BTreeMap::new();

        for line in CORPUS.lines() {
            if let Some(interior) = line
                .strip_prefix(start)
                .and_then(|line| line.strip_suffix(end))
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
        for line in CORPUS.lines() {
            if line.len() > 1 && core_set == set(&line[1..line.len() - 1]) {
                println!("{}", line);
            }
        }
    }
}

fn main() {
    Args::parse().run()
}
