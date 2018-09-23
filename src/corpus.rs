use crate::trie::{Trie};
use fnv::{FnvHashMap};

#[derive(Eq, PartialEq)]
enum TexParseMode {
    Init,
    Patterns,
    Exceptions
}

pub struct Corpus {
    patterns: Trie,
    exceptions: FnvHashMap<String, Vec<usize>>
}

impl Corpus {
    pub fn from_tex_file(path: &str) -> Result<Self, std::io::Error> {
        use std::fs::File;
        use std::io::{BufReader, prelude::*};

        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut line = String::new();
        let mut patterns = Trie::default();
        let mut exceptions = FnvHashMap::default();
        let mut mode = TexParseMode::Init;
        while let Ok(len) = buf_reader.read_line(&mut line) {
            if len == 0 {
                break;
            }
            let tline = line.trim();
            if tline.starts_with("\\patterns") {
                mode = TexParseMode::Patterns;
            } else if tline.starts_with("\\hyphenation") {
                mode = TexParseMode::Exceptions;
            } else if tline.starts_with("}") {
                mode = TexParseMode::Init;
            } else if mode == TexParseMode::Patterns && !tline.starts_with("#") {
                patterns.insert(&tline);
            } else if mode == TexParseMode::Exceptions && !tline.starts_with("#") {
                let indices = tline.to_string().char_indices().filter_map(|(index, c)| {
                    if c == '-' {
                        Some(index)
                    } else {
                        None
                    }
                }).collect::<Vec<_>>();
                exceptions.insert(tline.to_string().replace("-", ""), indices);
            }
            line.truncate(0);
        }

        Ok(Corpus {
            patterns,
            exceptions
        })
    }

    pub fn add_exception(&mut self, word: &str) {
        let indices = word.to_string().char_indices().filter_map(|(index, c)| {
            if c == '-' {
                Some(index)
            } else {
                None
            }
        }).collect::<Vec<_>>();
        self.exceptions.insert(word.to_string().replace("-", ""), indices);
    }

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
    fn creation_from_file() {
        let corpus = Corpus::from_file("./data/en");
        assert!(corpus.is_ok());
        assert_eq!(corpus.unwrap().get_pattern_count(), 4938);
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
