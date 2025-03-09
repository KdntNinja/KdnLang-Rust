use crate::parser::Expr;

// The Interpreter struct is responsible for evaluating expressions.
pub struct Interpreter;

impl Interpreter {
    // The visit method takes an expression and evaluates it.
    // It matches the expression type and calls the appropriate method.
    pub fn visit(&self, expr: &Expr) -> i32 {
        match expr {
            Expr::Number(n) => self.visit_number(*n),
            Expr::BinaryOp { left, op, right } => self.visit_binary_op(left, *op, right),
            Expr::Identifier(id) => self.visit_identifier(id),
        }
    }

    // The visit_binary_op method handles binary operations.
    // It evaluates the left and right expressions and applies the operator.
    fn visit_binary_op(&self, left: &Expr, op: char, right: &Expr) -> i32 {
        let left_val = self.visit(left);
        let right_val = self.visit(right);

        match op {
            '+' => left_val + right_val,
            '-' => left_val - right_val,
            '*' => left_val * right_val,
            '/' => left_val / right_val,
            _ => panic!("Unknown binary operator: {}", op),
        }
    }

    // The visit_number method handles number literals.
    // It simply returns the number.
    fn visit_number(&self, n: i32) -> i32 {
        n
    }

    // The visit_identifier method handles identifiers.
    // It currently returns a placeholder value.
    fn visit_identifier(&self, _id: &String) -> i32 {
        // Placeholder implementation
        0
    }
}
