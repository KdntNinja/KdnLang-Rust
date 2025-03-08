#[allow(dead_code)]
#[derive(Debug)]
pub enum Statement {
    Assignment {
        identifier: String,
        expression: Box<Expression>,
    },
    While {
        condition: Box<Expression>,
        body: Vec<Statement>,
    },
    ForLoop {
        variable: String,
        start_value: Box<Expression>,
        end_value: Box<Expression>,
        body: Vec<Statement>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Number(i64),
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    GreaterThan,
    LessThan,
    And,
    Or,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
