#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Statement {
    Assignment {
        identifier: String,
        expression: Box<Expression>,
    },
    Block {
        statements: Vec<Statement>,
        indentation: usize,
    },
    While {
        condition: Box<Expression>,
        body: Box<Statement>, // Changed to Box<Statement> to contain a Block
    },
    ForLoop {
        variable: String,
        start_value: Box<Expression>,
        end_value: Box<Expression>,
        body: Box<Statement>, // Changed to Box<Statement> to contain a Block
    },
    If {
        condition: Box<Expression>,
        then_branch: Box<Statement>, // Changed to Box<Statement> to contain a Block
        else_branch: Option<Box<Statement>>, // Changed to Option<Box<Statement>>
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Number(i64),
    String(String), // Add string literal support
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
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    And,
    Or,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}
