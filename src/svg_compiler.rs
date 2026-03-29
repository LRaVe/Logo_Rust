use crate::ast::{Ast, Number, Order, PenState};
use svg_fmt::{line_segment, red, BeginSvg, EndSvg};

pub struct LogoCompiler {
    x: f32,
    y: f32,
    angle: f32,
    pen: bool,
    width: f32,
    height: f32,
    body: String,
}

impl LogoCompiler {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            angle: 0.0,
            pen: true,
            width,
            height,
            body: String::new(),
        }
    }

    pub fn compile(&mut self, ast: &Ast) -> String {
        self.body.clear();
        self.walk(ast);

        let mut out = String::new();
        out.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        out.push_str(&format!("{}\n", BeginSvg { w: self.width, h: self.height }));
        out.push_str(&self.body);
        out.push_str(&format!("{}\n", EndSvg));
        out
    }

    fn walk(&mut self, ast: &Ast) {
        match ast {
            Ast::Program(command, next) => {
                self.walk(command);
                self.walk(next);
            }
            Ast::Command(order_ast, number_ast) => {
                if let (Some(order), Some(number)) = (extract_order(order_ast), extract_number(number_ast)) {
                    self.execute(order, number.0 as f32);
                }
            }
            Ast::State(state) => {
                self.pen = matches!(state, PenState::Down);
            }
            Ast::Loop(count_ast, body_ast) => {
                if let Some(count) = extract_number(count_ast) {
                    for _ in 0..count.0.max(0) {
                        self.walk(body_ast);
                    }
                }
            }
            Ast::Order(_) | Ast::Number(_) | Ast::Empty => {}
        }
    }

    fn execute(&mut self, order: Order, value: f32) {
        match order {
            Order::Forward => self.move_by(value),
            Order::Backward => self.move_by(-value),
            Order::Left => self.angle += value,
            Order::Right => self.angle -= value,
        }
    }

    fn move_by(&mut self, distance: f32) {
        let rad = self.angle.to_radians();
        let new_x = self.x + distance * rad.cos();
        let new_y = self.y - distance * rad.sin();

        if self.pen {
            let segment = line_segment(self.x, self.y, new_x, new_y).color(red()).width(2.0);
            self.body.push_str("    ");
            self.body.push_str(&format!("{}\n", segment));
        }

        self.x = new_x;
        self.y = new_y;
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
