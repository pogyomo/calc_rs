mod token;
mod lexer;
mod ident;
mod eval;

use std::io::{stdin, stdout, Write};
use eval::Eval;
use ident::Ident;
use lexer::Lexer;

fn main() {
    let mut ident_map = Ident::new();
    let cin = stdin();
    println!("------ Calc ------");
    for count in 1.. {
        print!("[{}]> ", count);
        stdout().flush().unwrap();
        let mut buf = String::new();
        match cin.read_line(&mut buf) {
            Ok(_)  => (),
            Err(_) => break,
        }

        let token = Lexer::new(buf.as_str()).tokenize();
        match Eval::new(token, &mut ident_map).eval() {
            Some(value) => println!("{}", value),
            None        => println!("Syntax error"),
        }
    }
}
