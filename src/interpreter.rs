//! Interpreter for KdnLang expressions.

use crate::parser::Expr;

/// Evaluates KdnLang expressions by traversing the AST.
pub struct Interpreter;

impl Interpreter {
    /// Evaluates an expression recursively.
    /// Returns the numeric result of the evaluation.
    pub fn visit(&self, expr: &Expr) -> i32 {
        match expr {
            Expr::Number(n) => *n,
            Expr::BinaryOp { left, op, right } => {
                let left_val = self.visit(left);
                let right_val = self.visit(right);
                
                match op {
                    '+' => left_val + right_val,
                    '-' => left_val - right_val,
                    '*' => left_val * right_val,
                    '/' => left_val / right_val,
                    _ => panic!("Unknown operator: {}", op),
                }
            },
            Expr::Identifier(_) => {
                // In a more advanced interpreter, this would look up variable values
                // For now, we'll return 0 for identifiers
                0
            }
        }
    }
}
