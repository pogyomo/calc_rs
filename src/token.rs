#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(u16),
    Ident(String),

    Let,      // Let

    LParenthesis, // '('
    RParenthesis, // ')'
    Equal,        // '='
    Add,          // '+'
    Sub,          // '-'
    Mul,          // '*'
    Div,          // '/'
}
