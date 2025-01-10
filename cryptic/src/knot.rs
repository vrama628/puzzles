use letter_boxed::trie::Trie;
use rand::{random, seq::SliceRandom, thread_rng};

use crate::knit::Knit;

pub struct Knot {
    trie: Trie,
    minimum_length: Option<usize>,
}

fn shuffled_chars() -> Vec<char> {
    let mut chars = ('a'..='z').collect::<Vec<_>>();
    chars.shuffle(&mut thread_rng());
    chars
}

impl Knot {
    pub fn knew(trie: Trie, minimum_length: Option<usize>) -> Self {
        Self {
            trie,
            minimum_length,
        }
    }

    fn f(&self, word: &Knit, words: &[&str], last_word: &Knit) {
        if let Some(last) = last_word
            .try_get()
            .filter(|&word| self.minimum_length.is_none_or(|n| word.len() >= n))
        {
            if let Some(word) = word.try_get().filter(|_| !words.is_empty()) {
                println!("{} {last} -> {word}", words.join(" "));
            }
            let mut words = words.to_vec();
            words.push(last);
            self.f(word, &words, &Knit::knew(&self.trie));
        }
        for c in shuffled_chars() {
            if let Some((word, last_word)) = word.push(c).zip(last_word.push(c)) {
                self.g(&word, words, &last_word)
            }
        }
    }

    fn g(&self, word: &Knit, words: &[&str], last_word: &Knit) {
        if let Some(last) = last_word
            .try_get()
            .filter(|&word| self.minimum_length.is_none_or(|n| word.len() >= n))
        {
            if let Some(word) = word.try_get().filter(|_| !words.is_empty()) {
                println!("{} {last} -> {word}", words.join(" "));
            }
            let mut words = words.to_vec();
            words.push(last);
            self.g(word, &words, &Knit::knew(&self.trie));
        }
        for c in shuffled_chars() {
            if let Some(last_word) = last_word.push(c) {
                self.f(&word, words, &last_word)
            }
        }
    }

    pub fn go(&self) {
        let word = Knit::knew(&self.trie);
        let last_word = Knit::knew(&self.trie);
        if random() {
            self.f(&word, &[], &last_word);
            self.g(&word, &[], &last_word);
        } else {
            self.g(&word, &[], &last_word);
            self.f(&word, &[], &last_word);
        }
    }
}
