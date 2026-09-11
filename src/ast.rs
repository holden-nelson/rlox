#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Unary {
        operator: UnaryOperator,
        rhs: Box<Expression>
    },
    Binary { 
        lhs: Box<Expression>, 
        operator: BinaryOperator, 
        rhs: Box<Expression>  
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Not
}

#[derive(Debug)]
pub enum BinaryOperator {
    Equals,
    NotEquals,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Plus,
    Minus,
    Times,
    Divide
}

trait ExpressionVisitor {
    type Output;

    fn visit_literal(&mut self, value: &Literal) -> Self::Output;

    fn visit_unary(
        &mut self,
        operator: &UnaryOperator,
        rhs: &Expression
    ) -> Self::Output;

    fn visit_binary(
        &mut self,
        lhs: &Expression,
        operator: &BinaryOperator,
        rhs: &Expression
    ) -> Self::Output;
}

impl Expression {
    fn accept<V: ExpressionVisitor>(
        &self, 
        visitor: &mut V
    ) -> V::Output {
        match self {
            Expression::Literal(value) => visitor.visit_literal(value),

            Expression::Unary { operator, rhs} => {
                visitor.visit_unary(operator, rhs)
            }

            Expression::Binary { 
                lhs, 
                operator, 
                rhs 
            } => visitor.visit_binary(lhs, operator, rhs),
            
        }
    }
}

struct AstPrinter;

impl ExpressionVisitor for AstPrinter {
    type Output = String;

    fn visit_literal(&mut self, value: &Literal) -> String {
        match value {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.clone(),
            Literal::True => "true".to_owned(),
            Literal::False => "false".to_owned(),
            Literal::Nil => "nil".to_owned()
        }
    }

    fn visit_unary(
        &mut self,
        operator: &UnaryOperator,
        rhs: &Expression
    ) -> String {
        format!("({operator:?} {})", rhs.accept(self))
    }

    fn visit_binary(
        &mut self,
        lhs: &Expression,
        operator: &BinaryOperator,
        rhs: &Expression
    ) -> String {
        format!(
            "({operator:?} {} {})",
            lhs.accept(self),
            rhs.accept(self)
        )
    }
}