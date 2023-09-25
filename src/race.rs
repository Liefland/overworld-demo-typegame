use crate::db::Text;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Race {
    pub line: String,
    pub source: String,
    pub input: String,
    pub start: Option<std::time::SystemTime>,
    pub end: Option<std::time::SystemTime>,
}

impl Race {
    pub fn with_line(line: Text) -> Self {
        Self {
            line: line.text,
            source: line.source,
            input: String::new(),
            start: None,
            end: None,
        }
    }

    pub fn score(&self) -> u64 {
        self.words_per_minute().mul(20.) as u64
    }

    pub fn insert_char(&mut self, character: char) -> bool {
        if !self.is_next(character) {
            return false;
        }

        if self.start.is_none() {
            self.start = Some(std::time::SystemTime::now());
        }

        let elapsed = self.start.expect("Should exist").elapsed();

        if elapsed.is_err() {
            return false;
        }

        self.input.push(character);

        if self.input == self.line {
            self.end = Some(std::time::SystemTime::now());
        }

        true
    }

    pub fn words_per_minute(&self) -> f32 {
        if self.start.is_none() {
            return 0.;
        }

        let word_count = self.input.split_whitespace().count();

        let (s, e) = (
            self.start.expect("Should exist"),
            self.end.unwrap_or_else(std::time::SystemTime::now),
        );

        let elapsed = e.duration_since(s).expect("Should be valid");

        word_count as f32 * elapsed.as_secs_f32()
    }

    fn is_next(&self, character: char) -> bool {
        self.line.as_bytes()[self.input.len()] as char == character
    }

    pub fn is_completed(&self) -> bool {
        self.end.is_some()
    }
}
