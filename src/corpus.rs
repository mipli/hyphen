use crate::trie::{Trie};

pub struct Corpus {
    trie: Trie
}

impl Corpus {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        use std::fs::File;
        use std::io::{BufReader, prelude::*};

        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        Ok(Corpus::from_string(&contents))
    }

    pub fn from_string(patterns: &str) -> Self {
        let mut corpus = Corpus {
            trie: Trie::new()
        };
        patterns.split_whitespace().for_each(|pattern| {
            corpus.trie.insert(pattern);
        });

        corpus
    }

    pub fn get_pattern_count(&self) -> usize {
        self.trie.count()
    }

    pub fn fetch(&self, chars: &[char]) -> Vec<usize> {
        self.trie.fetch(chars)
    }
}


#[cfg(test)]
mod tests {
    use super::{Corpus};
    #[test]
    fn creation_from_string() {
        let corpus = Corpus::from_string(".asdf e3f .ad5g");
        assert_eq!(corpus.get_pattern_count(), 3);
    }

    #[test]
    fn creation_from_file() {
        let corpus = Corpus::from_file("./data/en");
        assert!(corpus.is_ok());
        assert_eq!(corpus.unwrap().get_pattern_count(), 4938);
    }
}
