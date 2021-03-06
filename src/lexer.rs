#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // single character
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    Comma,
    Semicolon,
    ParenL,
    ParenR,
    BraceL,
    BraceR,
    // two character
    Eq,
    NotEq,
    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    // identifiers and literals
    Ident(String),
    Int(i32),
    // special
    EOF,
    Illegal,
}

impl Token {
    fn lookup_ident(input: &str) -> Token {
        match input {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(String::from(input)),
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        assert!(input.is_ascii());
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\u{0000}',
        };
        lexer.next_token();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::LT,
            '>' => Token::GT,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::ParenL,
            ')' => Token::ParenR,
            '{' => Token::BraceL,
            '}' => Token::BraceR,
            '\u{0000}' => Token::EOF,
            _ => {
                if Self::is_letter(self.ch) {
                    return self.read_keyword_or_identifier();
                } else if self.ch.is_digit(10) {
                    return self.read_number();
                } else {
                    Token::Illegal
                }
            }
        };
        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\u{0000}';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\u{0000}'
        } else {
            self.input[self.read_position]
        }
    }

    fn read_keyword_or_identifier(&mut self) -> Token {
        let position = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }
        let keyword_or_identifier: String = self.input[position..self.position].iter().collect();
        Token::lookup_ident(&keyword_or_identifier)
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        let number_string: String = self.input[position..self.position].iter().collect();
        Token::Int(number_string.parse().unwrap())
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
            let ten = 10;
            
            let add = fn(x, y) {
                x + y;
            };
            
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            
            10 == 10;
            10 != 9;";
        let expected_tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::ParenL,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::ParenR,
            Token::BraceL,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::BraceR,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::ParenL,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::ParenR,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::LT,
            Token::Int(10),
            Token::GT,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::ParenL,
            Token::Int(5),
            Token::LT,
            Token::Int(10),
            Token::ParenR,
            Token::BraceL,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::BraceR,
            Token::Else,
            Token::BraceL,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::BraceR,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for token in expected_tokens {
            let lexer_token = lexer.next_token();
            if lexer_token != token {
                panic!("mismatch! wanted: {:?}, got: {:?}", token, lexer_token);
            }
        }
    }
}
