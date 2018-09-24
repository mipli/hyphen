use crate::trie::{Trie};
use crate::tex_parser::{TexParser};
use fnv::{FnvHashMap};

pub struct Corpus {
    patterns: Trie,
    exceptions: FnvHashMap<String, Vec<usize>>,
    min_word_length: usize,
    left_min: usize,
    right_min: usize
}

impl Corpus {
    pub fn from_tex_file(path: &str) -> Result<Self, std::io::Error> {
        let corpus = Corpus::default();
        TexParser::parse_file(corpus, path)
    }

    pub fn min_word_length(&mut self, min: usize) {
        self.min_word_length = min;
    }

    pub fn left_min(&mut self, min: usize) {
        self.left_min = min;
    }

    pub fn right_min(&mut self, min: usize) {
        self.right_min = min;
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
        let mut corpus = Corpus::default();
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

    pub fn get_hyphenation_indices(&self, word: &str) -> Vec<usize> {
        if word.len() < self.min_word_length || word.len() <= self.left_min + self.right_min {
            return vec![];
        }
        if let Some(splits) = self.exceptions.get(word) {
            return splits.clone();
        }
        let mut chars = word.chars().collect::<Vec<_>>();
        chars.insert(0, '.');
        chars.push('.');
        let points = self.patterns.fetch(&chars);

        points.iter()
            .skip(1 + self.left_min)
            .take(1 + word.len() - self.left_min - self.right_min)
            .enumerate().filter_map(|(index, p)| {
                if *p > 0 && p % 2 == 0 {
                    Some(index + self.left_min)
                } else {
                    None
                }
            }).collect::<Vec<_>>()
    }
}

impl Default for Corpus {
    fn default() -> Self {
        Corpus {
            patterns: Trie::default(),
            exceptions: FnvHashMap::default(),
            min_word_length: 5,
            left_min: 2,
            right_min: 2
        }
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
