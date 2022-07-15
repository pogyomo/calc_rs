use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut ret = Vec::new();
        while let Some(token) = self.token() {
            ret.push(token);
        }
        ret
    }
}

impl<'a> Lexer<'a> {
    fn token(&mut self) -> Option<Token> {
        self.input = self.input.trim_start();
        if let Some(token) = self.ident_and_reserved_word() {
            Some(token)
        } else if let Some(token) = self.integer() {
            Some(token)
        } else if let Some(token) = self.single() {
            Some(token)
        } else {
            None
        }
    }

    fn ident_and_reserved_word(&mut self) -> Option<Token> {
        if self.input.chars().next()?.is_ascii_alphabetic() {
            let sep = self.input
                .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .unwrap_or(self.input.len());
            let ret    = &self.input[..sep];
            self.input = &self.input[sep..];

            match ret.to_lowercase().as_str() {
                "let" => Some(Token::Let),
                _ => Some(Token::Ident(ret.to_string())),
            }
        } else {
            None
        }
    }

    fn integer(&mut self) -> Option<Token> {
        if self.input.chars().next()?.is_ascii_digit() {
            let sep = self.input
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(self.input.len());
            let ret    = &self.input[..sep];
            self.input = &self.input[sep..];
            Some(Token::Integer(ret.parse::<u16>().ok()?))
        } else {
            None
        }
    }

    fn single(&mut self) -> Option<Token> {
        let mut chars = self.input.chars();
        if let Some(token) = match chars.next()? {
            '(' => Some(Token::LParenthesis),
            ')' => Some(Token::RParenthesis),
            '=' => Some(Token::Equal),
            '+' => Some(Token::Add),
            '-' => Some(Token::Sub),
            '*' => Some(Token::Mul),
            '/' => Some(Token::Div),
            _   => None,
        } {
            self.input = chars.as_str();
            Some(token)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::token::Token;

    #[test]
    fn test_lexer() {
        assert_eq!(
            Lexer::new("Value_temp = (10 + 20) / (4 * 2 - 3)").tokenize(),
            vec![
                Token::Ident("Value_temp".to_string()),
                Token::Equal,
                Token::LParenthesis,
                Token::Integer(10),
                Token::Add,
                Token::Integer(20),
                Token::RParenthesis,
                Token::Div,
                Token::LParenthesis,
                Token::Integer(4),
                Token::Mul,
                Token::Integer(2),
                Token::Sub,
                Token::Integer(3),
                Token::RParenthesis,
            ]);
    }
}
