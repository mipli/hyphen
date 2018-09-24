use crate::corpus::{Corpus};

#[derive(Eq, PartialEq)]
enum TexParseMode {
    Init,
    Patterns,
    Exceptions
}

pub struct TexParser {}

impl TexParser {
    pub fn parse_file(mut corpus: Corpus, path: &str) -> Result<Corpus, std::io::Error> {
        use std::fs::File;
        use std::io::{BufReader, prelude::*};

        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut line = String::new();
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
                corpus.add_pattern(&tline);
            } else if mode == TexParseMode::Exceptions && !tline.starts_with("#") {
                corpus.add_exception(&tline);
            }
            line.truncate(0);
        }
        Ok(corpus)
    }
}
