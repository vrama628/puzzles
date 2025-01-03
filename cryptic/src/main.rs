use clap::Parser;
use indexmap::IndexMap;
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
}

impl Args {
    fn run(self) {
        match self {
            Self::Case { start, end } => Self::case(start, end),
            Self::Core { core } => Self::core(core),
            Self::Knit => Self::knit(),
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
}

fn knit_knot(left: &Knit, right: &Knit, both: &Knit) {
    if left.trie.contains && right.trie.contains && both.trie.contains {
        println!("{} + {} = {}", left.prefix, right.prefix, both.prefix);
    }
    for c in 'A'..='Z' {
        if let Some((left, both)) = left.push(c).zip(both.push(c)) {
            knit_knot(right, &left, &both)
        }
    }
}

struct Knit<'a> {
    trie: &'a Trie,
    prefix: String,
}

impl<'a> Knit<'a> {
    fn knew(trie: &'a Trie) -> Self {
        Self {
            trie,
            prefix: String::new(),
        }
    }

    fn push(&self, c: char) -> Option<Self> {
        self.trie.children[c as usize % 32 - 1]
            .as_ref()
            .map(|trie| {
                let prefix = format!("{}{c}", self.prefix);
                Self { trie, prefix }
            })
    }
}

fn main() {
    Args::parse().run()
}
