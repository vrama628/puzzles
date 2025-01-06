pub struct Hide {
    target: String,
    starts: Vec<Vec<String>>,
    continuations: Vec<Vec<String>>,
}

// TODO: interior words

impl Hide {
    pub fn new(target: String) -> Self {
        let starts = vec![vec![]; target.len() + 1];
        let continuations = vec![vec![]; target.len() + 1];
        Self {
            target,
            starts,
            continuations,
        }
    }

    pub fn add(&mut self, word: &str) {
        for i in 1..word.len() {
            if self.target.starts_with(&word[i..]) {
                self.starts[word.len() - i].push(word.to_owned());
                for k in &self.continuations[word.len() - i] {
                    println!("{word} {k}")
                }
            }
            if self.target.ends_with(&word[..i]) {
                self.continuations[self.target.len() - i].push(word.to_owned());
                for start in &self.starts[self.target.len() - i] {
                    println!("{start} {word}")
                }
            }
        }
    }
}
