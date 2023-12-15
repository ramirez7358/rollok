use std::io::{Stdin, Stdout, Write};
use crate::lexer2::Lexer;
use crate::token::TokenKind;

pub fn start(stdin: Stdin, mut stdout: Stdout) {
    loop {
        write!(stdout, ">> ").expect("should have written prompt string >>");
        stdout.flush().expect("should have flushed stdout!");

        let mut input = String::new();
        if let Err(e) = stdin.read_line(&mut input) {
            writeln!(stdout, "Error: {e}").expect("should have written error message");
            return;
        }

        let mut lexer = Lexer::new(&input);

        loop {
            let token = lexer.next_token();
            if token.kind == TokenKind::EOF {
                break;
            }
            writeln!(stdout, "{token:?}").expect("Token should haven been written");
        }
    }
}