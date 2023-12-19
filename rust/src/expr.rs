// https://craftinginterpreters.com/representing-code.html

use crate::token::{Token, TokenType};

/// literal | unary | binary| grouping ;
pub enum Expr {
    // expression operator expression
    Binary(Box<Expr>, TokenType, Box<Expr>),
    // "(" expression ")" ;
    Grouping(Box<Expr>),
    // ( "-" | "!" ) expression ;
    Unary(TokenType, Box<Expr>),
    // NUMBER | STRING | "true" | "false" | "nil" ;
    Literal(Literal),
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

trait Visitor {
    fn visit_binary_expr(&mut self, left: &Box<Expr>, binary_op: &TokenType, right: &Box<Expr>);
    fn visit_grouping_expr(&mut self, expr: &Expr);
    fn visit_literal_expr(&mut self, literal: &Literal);
    fn visit_unary_expr(&mut self, unary: &TokenType, expr: &Box<Expr>);
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) {
        expr.accept(self)
    }
}

impl Expr {
    fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Expr::Binary(left, binary_op, right) => {
                visitor.visit_binary_expr(left, binary_op, right)
            }
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(literal) => visitor.visit_literal_expr(literal),
            Expr::Unary(unary, expr) => visitor.visit_unary_expr(unary, expr),
        }
    }
}

impl Visitor for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Box<Expr>, binary_op: &TokenType, right: &Box<Expr>) {
        print!("(");
        left.accept(self);
        match binary_op {
            TokenType::Minus => print!("-"),
            TokenType::Plus => print!("+"),
            TokenType::Slash => print!("/"),
            TokenType::Star => print!("*"),
            _ => {}
        }
        right.accept(self);
        print!(")");
    }
    fn visit_grouping_expr(&mut self, expr: &Expr) {
        print!("(");
        expr.accept(self);
        print!(")");
    }
    fn visit_literal_expr(&mut self, literal: &Literal) {
        match literal {
            Literal::Number(n) => print!("{}", n),
            Literal::String(s) => print!("{}", s),
            Literal::True => print!("true"),
            Literal::False => print!("false"),
            Literal::Nil => print!("nil"),
        }
    }
    fn visit_unary_expr(&mut self, unary: &TokenType, expr: &Box<Expr>) {
        print!("(");
        match unary {
            TokenType::Minus => print!("-"),
            TokenType::Bang => print!("!"),
            _ => {}
        }
        expr.accept(self);
        print!(")");
    }
}

fn main() {
    // -2 * (3 + 4)
    let tokens = Expr::Binary(
        Box::new(Expr::Unary(
            TokenType::Minus,
            Box::new(Expr::Literal(Literal::Number(2.0))),
        )),
        TokenType::Star,
        Box::new(Expr::Grouping(Box::new(Expr::Binary(
            Box::new(Expr::Literal(Literal::Number(3.0))),
            TokenType::Plus,
            Box::new(Expr::Literal(Literal::Number(4.0))),
        )))),
    );
    let mut printer = AstPrinter;
    printer.print(&tokens);
}
