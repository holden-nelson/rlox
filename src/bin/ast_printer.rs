#[allow(dead_code)]
#[path = "../ast.rs"]
mod ast;

use ast::{AstPrinter, BinaryOperator, Expression, Literal, UnaryOperator};

fn main() {
    // Represents: -123 * 45.67
    let expression = Expression::Binary {
        lhs: Box::new(Expression::Unary {
            operator: UnaryOperator::Negate,
            rhs: Box::new(Expression::Literal(Literal::Number(123.0))),
        }),
        operator: BinaryOperator::Times,
        rhs: Box::new(Expression::Literal(Literal::Number(45.67))),
    };

    let mut printer = AstPrinter;
    println!("{}", printer.print(&expression));
}
