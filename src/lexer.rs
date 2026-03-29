use santiago::lexer::LexerRules;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme {
    pub kind: String,
    pub raw: String,
    pub position: (usize, usize),
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} \"{}\" at ({}, {})",
            self.kind, self.raw, self.position.0, self.position.1
        )
    }
}

#[allow(dead_code)]
pub struct LogoLexer {}

#[allow(dead_code)]
pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "FORWARD" = pattern r"(?i:forward)\b";
        "DEFAULT" | "BACKWARD" = pattern r"(?i:backward)\b";
        "DEFAULT" | "LEFT" = pattern r"(?i:left)\b";
        "DEFAULT" | "RIGHT" = pattern r"(?i:right)\b";
        "DEFAULT" | "STATE" = pattern r"(?i:penup|pendown)\b";
        "DEFAULT" | "LOOP" = pattern r"(?i:repeat)\b";
        "DEFAULT" | "LBRACKET" = pattern r"\[";
        "DEFAULT" | "RBRACKET" = pattern r"\]";
        "DEFAULT" | "NUMBER" = pattern r"[0-9]+";
        "DEFAULT" | "WS" = pattern r"\s+" => |lexer| lexer.skip();
    )
}

impl LogoLexer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LogoLexer {}
    }

    #[allow(dead_code)]
    pub fn lex(&self, input: &str) -> Vec<Lexeme> {
        let rules = lexer_rules();

        match santiago::lexer::lex(&rules, input) {
            Ok(lexemes) => lexemes
                .iter()
                .map(|l| Lexeme {
                    kind: l.kind.clone(),
                    raw: l.raw.clone(),
                    position: (l.position.line, l.position.column),
                })
                .collect(),
            Err(err) => {
                eprintln!("Lexing error: {}", err);
                Vec::new()
            }
        }
    }
}

