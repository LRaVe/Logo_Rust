// Logo Lexer Module
// Simple lexical analyzer for Logo language using regex

use regex::Regex;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme {
    pub kind: String,
    pub raw: String,
    pub position: (usize, usize), // (line, column)
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} \"{}\" at ({}, {})", self.kind, self.raw, self.position.0, self.position.1)
    }
}

pub struct LogoLexer {}

impl LogoLexer {
    pub fn new() -> Self {
        LogoLexer {}
    }

    pub fn lex(&self, input: &str) -> Vec<Lexeme> {
        let mut lexemes = Vec::new();
        let mut line = 1;
        let mut column = 1;

        let keyword_regex = Regex::new(r"(forward|backward|left|right)").unwrap();
        let number_regex = Regex::new(r"[0-9]+").unwrap();
        let whitespace_regex = Regex::new(r"\s+").unwrap();

        let mut current_pos = 0;

        while current_pos < input.len() {
            let remaining = &input[current_pos..];

            // Try to match whitespace
            if let Some(m) = whitespace_regex.find(remaining) {
                if m.start() == 0 {
                    let ws = m.as_str();
                    for ch in ws.chars() {
                        if ch == '\n' {
                            line += 1;
                            column = 1;
                        } else {
                            column += 1;
                        }
                    }
                    current_pos += ws.len();
                    continue;
                }
            }

            // Try to match keywords
            if let Some(m) = keyword_regex.find(remaining) {
                if m.start() == 0 {
                    let keyword = m.as_str();
                    let kind = keyword.to_uppercase();
                    lexemes.push(Lexeme {
                        kind,
                        raw: keyword.to_string(),
                        position: (line, column),
                    });
                    column += keyword.len();
                    current_pos += keyword.len();
                    continue;
                }
            }

            // Try to match numbers
            if let Some(m) = number_regex.find(remaining) {
                if m.start() == 0 {
                    let number = m.as_str();
                    lexemes.push(Lexeme {
                        kind: "NUMBER".to_string(),
                        raw: number.to_string(),
                        position: (line, column),
                    });
                    column += number.len();
                    current_pos += number.len();
                    continue;
                }
            }

            // Unknown character - skip it
            if let Some(ch) = remaining.chars().next() {
                column += 1;
                current_pos += ch.len_utf8();
            }
        }

        lexemes
    }
}

