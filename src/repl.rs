use super::lexer::Lexer;
use super::lexer::Token;
use std::io;
use std::io::Write;

pub fn start() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let mut lexer = Lexer::new(&input);
        loop {
            let token = lexer.next_token();
            if token == Token::EOF {
                break;
            }
            println!("{:?}", token);
        }
    }
}
