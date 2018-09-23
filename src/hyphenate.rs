use crate::corpus::{Corpus};
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

pub trait Hyphenate<'a> {
    fn possibilities_word(self, corpus: &Corpus) -> Vec<usize>;
    fn mark_word(self, _corpus: &Corpus) -> StandardHyphenator<'a>;

    fn possibilities(self, corpus: &Corpus) -> Vec<usize>;
    fn mark(self, _corpus: &Corpus) -> StandardHyphenator<'a>;
}

impl<'a> Hyphenate<'a> for &'a str {
    fn possibilities_word(self, corpus: &Corpus) -> Vec<usize> {
        let mut chars = self.chars().collect::<Vec<_>>();
        chars.insert(0, '.');
        chars.push('.');
        let points = corpus.fetch(&chars);

        points.iter().skip(1).enumerate().filter_map(|(index, p)| {
            if *p > 0 && p % 2 == 0 {
                Some(index)
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }

    fn mark_word(self, corpus: &Corpus) -> StandardHyphenator<'a> {
        StandardHyphenator {
            text: self,
            possibilities: self.possibilities_word(corpus),
            prior: 0,
            current: 0
        }
    }

    fn possibilities(self, corpus: &Corpus) -> Vec<usize> {
        let words = self.split_word_bound_indices();
        words.flat_map(|(i, word)| {
            word.possibilities_word(corpus).into_iter().map(move |j| i + j)
        }).collect()
    }

    fn mark(self, corpus: &Corpus) -> StandardHyphenator<'a> {
        StandardHyphenator {
            text: self,
            possibilities: self.possibilities(corpus),
            prior: 0,
            current: 0
        }
    }
}

#[derive(Debug)]
pub struct StandardHyphenator<'a> {
    text: &'a str,
    possibilities: Vec<usize>,
    prior: usize,
    current: usize
}

impl<'a> StandardHyphenator<'a> {
    pub fn hyphenate(self) ->  String {
        self.hyphenate_with("\u{ad}")
    }

    pub fn hyphenate_with(self, mark: &'a str) ->  String {
        self.intersperse(mark).collect::<String>()
    }
}

impl<'a> Iterator for StandardHyphenator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let start = self.prior;
        let current = self.current;

        match self.possibilities.get(current) {
            Some(&pos) => {
                self.prior = pos;
                self.current += 1;
                Some(&self.text[start..pos])
            },
            None if current <= self.possibilities.len() => {
                self.current += 1;
                Some(&self.text[start..])
            },
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Corpus, Hyphenate};

    #[test]
    fn single_word_hyphenation() {
        let corpus = Corpus::from_string(".as4d8f");
        let hyphenator = "asdf".mark_word(&corpus);
        let split = hyphenator.hyphenate();
        assert_eq!(split, "as\u{ad}d\u{ad}f");
    }

    #[test]
    fn custom_hyphenation_mark() {
        let corpus = Corpus::from_string(".asd8f");
        let hyphenator = "asdf".mark_word(&corpus);
        let split = hyphenator.hyphenate_with("<shy></shy>");
        assert_eq!(split, "asd<shy></shy>f");
    }

    #[test]
    fn fulltext_hyphenation() {
        let corpus = Corpus::from_string(".as4d8f");
        let hyphenator = "asdf foo asdf".mark(&corpus);
        let split = hyphenator.hyphenate();
        assert_eq!(split, "as\u{ad}d\u{ad}f foo as\u{ad}d\u{ad}f");
    }

    #[test]
    fn negation_hyphenation() {
        let corpus = Corpus::from_string(".as4df s9d asd4f");
        let hyphenator = "asdf".mark(&corpus);
        assert_eq!(hyphenator.hyphenate(), "asd\u{ad}f");
    }
}
