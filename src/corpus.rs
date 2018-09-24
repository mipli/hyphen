use crate::trie::{Trie};
use crate::tex_parser::{TexParser};
use fnv::{FnvHashMap};

pub struct Corpus {
    patterns: Trie,
    exceptions: FnvHashMap<String, Vec<usize>>
}

impl Corpus {
    pub fn from_tex_file(path: &str) -> Result<Self, std::io::Error> {
        let corpus = Corpus {
            patterns: Trie::default(),
            exceptions: FnvHashMap::default()
        };
        TexParser::parse_file(corpus, path)
    }

    pub fn add_pattern(&mut self, pattern: &str) {
        self.patterns.insert(pattern);
    }

    pub fn add_exception(&mut self, word: &str) {
        let word = word.to_string();
        let indices = word.char_indices().filter_map(|(index, c)| {
            if c == '-' {
                Some(index)
            } else {
                None
            }
        }).collect::<Vec<_>>();
        self.exceptions.insert(word.replace("-", ""), indices);
    }

    pub fn from_string(patterns: &str) -> Self {
        let mut corpus = Corpus {
            patterns: Trie::default(),
            exceptions: FnvHashMap::default()
        };
        patterns.split_whitespace().for_each(|pattern| {
            corpus.patterns.insert(pattern);
        });

        corpus
    }

    pub fn get_pattern_count(&self) -> usize {
        self.patterns.count()
    }

    pub fn get_exception_count(&self) -> usize {
        self.exceptions.len()
    }

    pub fn fetch(&self, chars: &[char]) -> Vec<usize> {
        self.patterns.fetch(chars)
    }

    pub fn get_exception(&self, word: &str) -> Option<&Vec<usize>> {
        self.exceptions.get(word)
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
    fn creation_from_tex_file() {
        let corpus = Corpus::from_tex_file("./data/hyph-en-gb.tex");
        assert!(corpus.is_ok());
        let corpus = corpus.unwrap();
        assert_eq!(corpus.get_pattern_count(), 8527);
        assert_eq!(corpus.get_exception_count(), 8);
    }
}
