use santiago::parser::Tree;

#[derive(Debug, Clone, PartialEq)]
pub enum Order {
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Number(pub i32);

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Program(Box<Ast>, Box<Ast>),
    Command(Box<Ast>, Box<Ast>),
    Order(Order),
    Number(Number),
    Empty,
}

pub fn from_parse_tree(tree: &Tree<()>) -> Result<Ast, String> {
    match tree {
        Tree::Node {
            rule_name,
            leaves,
            production: _,
        } => match rule_name.as_str() {
            // Santiago injects an augmented start rule.
            "ɣ" => {
                if leaves.len() == 1 {
                    from_parse_tree(&leaves[0])
                } else {
                    Err("Invalid root rule shape".to_string())
                }
            }
            "program" => {
                if leaves.is_empty() {
                    Ok(Ast::Empty)
                } else if leaves.len() == 2 {
                    let command = from_parse_tree(&leaves[0])?;
                    let next = from_parse_tree(&leaves[1])?;
                    Ok(Ast::Program(Box::new(command), Box::new(next)))
                } else {
                    Err("Invalid program rule shape".to_string())
                }
            }
            "command" => {
                if leaves.len() != 2 {
                    return Err("Invalid command rule shape".to_string());
                }
                let order = from_parse_tree(&leaves[0])?;
                let number = from_parse_tree(&leaves[1])?;
                Ok(Ast::Command(Box::new(order), Box::new(number)))
            }
            "order" | "number" => {
                if leaves.len() == 1 {
                    from_parse_tree(&leaves[0])
                } else {
                    Err(format!("Invalid {} rule shape", rule_name))
                }
            }
            _ => {
                if leaves.len() == 1 {
                    from_parse_tree(&leaves[0])
                } else {
                    Err(format!("Unsupported rule: {}", rule_name))
                }
            }
        },
        Tree::Leaf(lexeme) => match lexeme.kind.as_str() {
            "FORWARD" => Ok(Ast::Order(Order::Forward)),
            "BACKWARD" => Ok(Ast::Order(Order::Backward)),
            "LEFT" => Ok(Ast::Order(Order::Left)),
            "RIGHT" => Ok(Ast::Order(Order::Right)),
            "NUMBER" => {
                let value = lexeme
                    .raw
                    .parse::<i32>()
                    .map_err(|e| format!("Invalid number '{}': {}", lexeme.raw, e))?;
                Ok(Ast::Number(Number(value)))
            }
            other => Err(format!("Unsupported lexeme kind: {}", other)),
        },
    }
}

pub fn eval(ast: &Ast) {
    match ast {
        Ast::Program(command, next) => {
            eval(command);
            eval(next);
        }
        Ast::Command(order_ast, number_ast) => {
            let order = extract_order(order_ast);
            let number = extract_number(number_ast);
            if let (Some(order), Some(number)) = (order, number) {
                match order {
                    Order::Forward => println!("Advance {} units", number.0),
                    Order::Backward => println!("Move back {} units", number.0),
                    Order::Left => println!("Turn left {} degrees", number.0),
                    Order::Right => println!("Turn right {} degrees", number.0),
                }
            }
        }
        Ast::Order(_) | Ast::Number(_) => {}
        Ast::Empty => println!("Stop"),
    }
}

fn extract_order(ast: &Ast) -> Option<Order> {
    match ast {
        Ast::Order(order) => Some(order.clone()),
        _ => None,
    }
}

fn extract_number(ast: &Ast) -> Option<Number> {
    match ast {
        Ast::Number(number) => Some(*number),
        _ => None,
    }
}
