// Logo Parser Module
// Syntactic analysis using Santiago grammar library

use std::rc::Rc;
use santiago::grammar::Grammar;
use santiago::parser::{self, Tree};
use crate::lexer::Lexeme;

/// Define the Logo grammar using Santiago
pub fn grammar() -> Grammar<()> {
    santiago::grammar!(
        "program" => rules "command" "program"
                   ;
        "program" => empty
                   ;
        "command" => rules "order" "number"
                   ;
        "order" => lexemes "FORWARD" 
                ;
        "order" => lexemes "BACKWARD"
                ;
        "order" => lexemes "LEFT"
                ;
        "order" => lexemes "RIGHT"
                ;
        "number" => lexemes "NUMBER"
                  ;
    )
}

/// Convert lexemes to Santiago format (Rc-wrapped)
pub fn prepare_lexemes(lexemes: &[Lexeme]) -> Vec<Rc<santiago::lexer::Lexeme>> {
    lexemes.iter().map(|l| {
        Rc::new(santiago::lexer::Lexeme {
            kind: l.kind.clone(),
            raw: l.raw.clone(),
            position: santiago::lexer::Position {
                line: l.position.0,
                column: l.position.1,
            },
        })
    }).collect()
}

/// Parse lexeme sequence and return the parse trees
pub fn parse(lexemes: &[Lexeme]) -> Result<Vec<Rc<Tree<()>>>, String> {
    let grammar = grammar();
    let santiago_lexemes = prepare_lexemes(lexemes);
    
    match parser::parse(&grammar, &santiago_lexemes) {
        Ok(trees) => Ok(trees),
        Err(e) => Err(format!("Parse error: {}", e))
    }
}

pub fn parse_first(lexemes: &[Lexeme]) -> Result<Rc<Tree<()>>, String> {
    let trees = parse(lexemes)?;
    trees
        .into_iter()
        .next()
        .ok_or_else(|| "No parse tree found".to_string())
}

/// Pretty-print the parse tree
pub fn print_parse_tree(tree: &Tree<()>, indent: usize) {
    let indent_str = "  ".repeat(indent);
    match tree {
        Tree::Node { rule_name, production: _, leaves } => {
            println!("{}|- Rule: {}", indent_str, rule_name);
            for leaf in leaves {
                print_parse_tree(leaf, indent + 1);
            }
        }
        Tree::Leaf(lexeme) => {
            println!("{}|- {}: \"{}\"", indent_str, lexeme.kind, lexeme.raw);
        }
    }
}
