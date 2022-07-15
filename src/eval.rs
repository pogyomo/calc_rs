use std::process::exit;
use crate::token::Token;
use crate::Ident;

pub struct Eval<'a> {
    token: Vec<Token>,
    ident: &'a mut Ident,
    pos: usize,
}

impl<'a> Eval<'a> {
    pub fn new(token: Vec<Token>, ident: &'a mut Ident) -> Eval<'a> {
        Eval { token, ident, pos: 0 }
    }

    pub fn eval(&mut self) -> Option<u16> {
        match self.token.get(self.pos)? {
            Token::Let => {
                self.pos += 1;
                self.ident()
            }
            Token::Exit => {
                println!("Good Bye!");
                exit(0);
            }
            Token::List => {
                let mut count = 0;
                for (name, value) in self.ident.map.iter() {
                    count += 1;
                    println!("{} = {}", name, value);
                }
                Some(count)
            }
            _ => self.eval1()
        }
    }
}

impl<'a> Eval<'a> {
    fn ident(&mut self) -> Option<u16> {
        if let Token::Ident(ref ident) = *self.token.get(self.pos)? {
            self.pos += 1;
            if Token::Equal == *self.token.get(self.pos)? {
                let ident = ident.to_string();
                self.pos += 1;
                let value = self.eval1()?;
                self.ident.register(ident, value);
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn eval1(&mut self) -> Option<u16> {
        let mut left = self.eval2()?;
        loop {
            if let Some(op) = self.token.get(self.pos) {
                if *op == Token::Add {
                    self.pos += 1;
                    left += self.eval2()?;
                } else if *op == Token::Sub {
                    self.pos += 1;
                    left -= self.eval2()?;
                } else {
                    break Some(left);
                }
            } else {
                break Some(left);
            }
        }
    }

    fn eval2(&mut self) -> Option<u16> {
        let mut left = self.eval3()?;
        loop {
            if let Some(op) = self.token.get(self.pos) {
                if *op == Token::Mul {
                    self.pos += 1;
                    left *= self.eval3()?;
                } else if *op == Token::Div {
                    self.pos += 1;
                    left /= self.eval3()?;
                } else {
                    break Some(left);
                }
            } else {
                break Some(left);
            }
        }
    }

    fn eval3(&mut self) -> Option<u16> {
        if *self.token.get(self.pos)? == Token::LParenthesis {
            self.pos += 1;
            let value = self.eval1()?;
            if *self.token.get(self.pos)? == Token::RParenthesis {
                self.pos += 1;
                Some(value)
            } else {
                None
            }
        } else if let Token::Integer(value) = *self.token.get(self.pos)?  {
            self.pos += 1;
            Some(value)
        } else if let Token::Ident(ref ident) = *self.token.get(self.pos)? {
            self.pos += 1;
            Some(self.ident.value(ident)?)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{token::Token, ident::Ident};
    use super::Eval;

    #[test]
    fn test_eval() {
        let token = vec![
            Token::Ident("Value_Ident".to_string()),
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
        ];

        let mut ident = Ident::new();
        assert_eq!(Eval::new(token, &mut ident).eval(), Some(6));
    }
}
