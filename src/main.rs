mod token;
mod lexer;
mod ident;
mod eval;

use std::io::stdin;
use eval::Eval;
use ident::Ident;
use lexer::Lexer;

fn main() {
    let mut ident_map = Ident::new();
    let cin = stdin();
    loop {
        let mut buf = String::new();
        match cin.read_line(&mut buf) {
            Ok(_)  => (),
            Err(_) => break,
        }
        let token = Lexer::new(buf.as_str()).tokenize();
        match Eval::new(token, &mut ident_map).eval() {
            Some(value) => println!("result -> {}", value),
            None        => break,
        }
    }
}
