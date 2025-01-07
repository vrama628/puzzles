pub struct Hide {
    target: String,
    starts: Vec<Vec<String>>,
    middles: Vec<Vec<Option<String>>>,
    ends: Vec<Vec<String>>,
}

impl Hide {
    pub fn new(target: String) -> Self {
        let starts = vec![vec![]; target.len() + 1];
        let middles = vec![vec![None; target.len() + 1]; target.len() + 1];
        let ends = vec![vec![]; target.len() + 1];
        Self {
            target,
            starts,
            middles,
            ends,
        }
    }

    fn prefixes(&self, i: usize) -> Vec<Vec<String>> {
        let mut results: Vec<Vec<String>> = self.starts[i]
            .iter()
            .map(|start| vec![start.clone()])
            .collect();
        for j in 0..i {
            if let Some(middle) = &self.middles[j][i] {
                let mut prefixes = self.prefixes(j);
                for prefix in &mut prefixes {
                    prefix.push(middle.clone());
                }
                results.extend(prefixes)
            }
        }
        results
    }

    fn suffixes(&self, i: usize) -> Vec<Vec<String>> {
        let mut results: Vec<Vec<String>> =
            self.ends[i].iter().map(|end| vec![end.clone()]).collect();
        for j in i + 1..self.target.len() {
            if let Some(middle) = &self.middles[i][j] {
                let mut suffixes = self.suffixes(j);
                for suffix in &mut suffixes {
                    suffix.insert(0, middle.clone());
                }
                results.extend(suffixes)
            }
        }
        results
    }

    pub fn add(&mut self, word: &str) {
        for (i, _) in self.target.match_indices(word) {
            if i == 0 || i + word.len() == self.target.len() {
                continue;
            }
            let clobbered = self.middles[i][i + word.len()]
                .replace(word.to_owned())
                .is_some();
            debug_assert!(!clobbered);
            for prefix in &mut self.prefixes(i) {
                for suffix in &mut self.suffixes(i + word.len()) {
                    println!("{} {word} {}", prefix.join(" "), suffix.join(" "))
                }
            }
        }
        for i in 1..word.len() {
            if self.target.starts_with(&word[i..]) {
                self.starts[word.len() - i].push(word.to_owned());
                for suffix in &self.suffixes(word.len() - i) {
                    println!("{word} {}", suffix.join(" "))
                }
            }
            if self.target.ends_with(&word[..i]) {
                self.ends[self.target.len() - i].push(word.to_owned());
                for prefix in &self.prefixes(self.target.len() - i) {
                    println!("{} {word}", prefix.join(" "))
                }
            }
        }
    }
}
