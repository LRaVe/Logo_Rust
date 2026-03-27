use std::fs;

mod lexer;
mod parser;
mod ast;
mod svg_compiler;

use lexer::LogoLexer;
use svg_compiler::LogoCompiler;

fn main() {
    // Read the Logo program from square.logo file
    let input = match fs::read_to_string("square.logo") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    println!("Input program:");
    println!("{}\n", input);

    
    let lexer = LogoLexer::new();
    let tokens = lexer.lex(&input);
    
    println!("Tokens extracted:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  [{}] {} \"{}\" at ({}, {})", 
                 i, token.kind, token.raw, token.position.0, token.position.1);
    }
    println!("Total tokens: {}\n", tokens.len());

    
    match parser::parse(&tokens) {
        Ok(trees) => {
            if trees.is_empty() {
                eprintln!("No valid parse tree found\n");
            } else {
                println!("OK Syntax valid - program structure is correct");
                println!("Found {} parse tree(s)\n", trees.len());

                for (i, tree) in trees.iter().enumerate() {
                    println!("Parse Tree {}:", i + 1);
                    parser::print_parse_tree(tree, 0);
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("X Syntax error: {}\n", e);
            return;
        }
    }


    let parse_tree = match parser::parse_first(&tokens) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("X Cannot build AST without parse tree: {}", e);
            return;
        }
    };

    let ast = match ast::from_parse_tree(&parse_tree) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("X AST build error: {}", e);
            return;
        }
    };

    println!("AST:");
    println!("{:#?}\n", ast);

    println!("Eval:");
    ast::eval(&ast);


    let mut compiler = LogoCompiler::new(300.0, 300.0);
    let svg = compiler.compile(&ast);

    let output_path = "output.svg";
    match fs::write(output_path, svg) {
        Ok(_) => println!("SVG generated: {}", output_path),
        Err(e) => {
            eprintln!("X Failed to write {}: {}", output_path, e);
            return;
        }
    }

}
