use santiago::parser::Tree;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Order {
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum PenState {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub struct Number(pub i32);

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Ast {
    Program(Box<Ast>, Box<Ast>),
    Command(Box<Ast>, Box<Ast>),
    State(PenState),
    Loop(Box<Ast>, Box<Ast>),
    Order(Order),
    Number(Number),
    Empty,
}

#[allow(dead_code)]
pub fn from_parse_tree(tree: &Tree<()>) -> Result<Ast, String> {
    match tree {
        Tree::Node {
            rule_name,
            leaves,
            production: _,
        } => match rule_name.as_str() {
            
            "Γ" => {
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
                if leaves.len() == 1 {
                    from_parse_tree(&leaves[0])
                } else {
                    Err("Invalid command rule shape".to_string())
                }
            }
            "move" => {
                if leaves.len() != 2 {
                    return Err("Invalid move rule shape".to_string());
                }
                let order = from_parse_tree(&leaves[0])?;
                let number = from_parse_tree(&leaves[1])?;
                Ok(Ast::Command(Box::new(order), Box::new(number)))
            }
            "loop" => {
                if leaves.len() != 5 {
                    return Err("Invalid loop rule shape".to_string());
                }
                let count = from_parse_tree(&leaves[1])?;
                let body = from_parse_tree(&leaves[3])?;
                Ok(Ast::Loop(Box::new(count), Box::new(body)))
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
            "STATE" => match lexeme.raw.to_ascii_lowercase().as_str() {
                "penup" => Ok(Ast::State(PenState::Up)),
                "pendown" => Ok(Ast::State(PenState::Down)),
                other => Err(format!("Unsupported state command: {}", other)),
            },
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

#[allow(dead_code)]
pub fn eval(ast: &Ast) {
    match ast {
        Ast::Program(command, suivant) => {
            eval(command);
            eval(suivant);
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
        Ast::State(state) => match state {
            PenState::Up => println!("Pen up"),
            PenState::Down => println!("Pen down"),
        },
        Ast::Loop(count_ast, body_ast) => {
            if let Some(count) = extract_number(count_ast) {
                for _ in 0..count.0.max(0) {
                    eval(body_ast);
                }
            }
        }
        Ast::Order(_) | Ast::Number(_) => {}
        Ast::Empty => println!("Stop"),
    }
}

#[allow(dead_code)]
fn extract_order(ast: &Ast) -> Option<Order> {
    match ast {
        Ast::Order(order) => Some(order.clone()),
        _ => None,
    }
}

#[allow(dead_code)]
fn extract_number(ast: &Ast) -> Option<Number> {
    match ast {
        Ast::Number(number) => Some(*number),
        _ => None,
    }
}
