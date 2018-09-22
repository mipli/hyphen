use crate::trie::{Trie};

pub struct Hyphen {
    trie: Trie,
    delimiter: char
}

impl Hyphen {
    fn new() -> Self {
        Hyphen {
            trie: Trie::default(),
            delimiter: '-'
        }
    }

    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        use std::fs::File;
        use std::io::{BufReader, prelude::*};

        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        Ok(Hyphen::from_string(&contents))
    }

    pub fn from_string(patterns: &str) -> Self {
        let mut hyphen = Hyphen::new();
        patterns.split_whitespace().for_each(|pattern| {
            hyphen.trie.insert(pattern);
        });

        hyphen
    }

    pub fn get_pattern_count(&self) -> usize {
        self.trie.count()
    }

    pub fn hyphenate_word<'a>(&self, word: &'a str) -> HyphenatedWord<'a> {
        let mut chars = word.chars().collect::<Vec<_>>();
        chars.insert(0, '.');
        chars.push('.');
        let points = self.trie.fetch(&chars);

        let split_points = points.iter().skip(1).enumerate().filter_map(|(index, p)| {
            if *p > 0 && p % 2 == 0 {
                Some(index)
            } else {
                None
            }
        });

        let (mut parts, rem, _) = split_points.fold((vec![], word, 0), |(mut parts, word, offset), point| {
            let (word, rem) = word.split_at(point - offset);
            parts.push(word);
            (parts, rem, point)
        });
        parts.push(rem);
        HyphenatedWord {
            parts,
            delimiter: self.delimiter
        }
    }
}

pub struct HyphenatedWord<'a> {
    parts: Vec<&'a str>,
    delimiter: char
}

impl<'a> From<HyphenatedWord<'a>> for String {
    fn from(word: HyphenatedWord<'a>) -> Self {
        word.parts.join(&word.delimiter.to_string()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{Hyphen};

    #[test]
    fn creation_from_string() {
        let hyphen = Hyphen::from_string(".asdf e3f .ad5g");
        assert_eq!(hyphen.get_pattern_count(), 3);
    }

    #[test]
    fn creation_from_file() {
        let hyphen = Hyphen::from_file("./data/en");
        assert!(hyphen.is_ok());

        let hyphen = hyphen.unwrap();
        assert_eq!(hyphen.get_pattern_count(), 4938);
    }

    #[test]
    fn single_hyphenate_word() {
        let hyphen = Hyphen::from_string(".as4df");
        assert_eq!(hyphen.hyphenate_word("asdf").parts, vec!["as", "df"]);
    }

    #[test]
    fn double_hyphenate_word() {
        let hyphen = Hyphen::from_string(".as4df sd6f");
        assert_eq!(hyphen.hyphenate_word("asdf").parts, vec!["as", "d", "f"]);
    }

    #[test]
    fn hyphenation_negation() {
        let hyphen = Hyphen::from_string(".as4df s9d asd4f");
        assert_eq!(hyphen.hyphenate_word("asdf").parts, vec!["asd", "f"]);
    }

    #[test]
    fn string_conversion() {
        let hyphen = Hyphen::from_string(".as4df sd6f");
        let word: String = hyphen.hyphenate_word("asdf").into();
        assert_eq!(word, "as-d-f");
    }
}
