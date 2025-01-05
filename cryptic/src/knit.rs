use std::rc::Rc;

use letter_boxed::trie::Trie;

pub fn knit_knot(left: &Knit, right: &Knit, both: &Knit) {
    if let Some(((l, r), b)) = left.try_get().zip(right.try_get()).zip(both.try_get()) {
        println!("{l} + {r} = {b}");
    }
    for c in 'A'..='Z' {
        if let Some((left, both)) = left.push(c).zip(both.push(c)) {
            knit_knot(right, &left, &both)
        }
    }
}

#[derive(Clone)]
pub struct Knit<'a> {
    trie: &'a Trie,
    prefix: Rc<str>,
}

impl<'a> Knit<'a> {
    pub fn knew(trie: &'a Trie) -> Self {
        Self {
            trie,
            prefix: Rc::from(""),
        }
    }

    pub fn push(&self, c: char) -> Option<Self> {
        self.trie.children[c as usize % 32 - 1]
            .as_ref()
            .map(|trie| {
                let prefix = Rc::from(format!("{}{c}", self.prefix));
                Self { trie, prefix }
            })
    }

    pub fn try_get(&self) -> Option<&str> {
        if self.trie.contains {
            Some(&self.prefix)
        } else {
            None
        }
    }
}
