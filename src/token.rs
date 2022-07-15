#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(u16),
    Ident(String),

    Let,  // Let
    Exit, // Exit
    List, // List all variable

    LParenthesis, // '('
    RParenthesis, // ')'
    Equal,        // '='
    Add,          // '+'
    Sub,          // '-'
    Mul,          // '*'
    Div,          // '/'
}
