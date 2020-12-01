use super::ast::{Identifier, LetStatement, Program, Statement};
use super::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: Vec::new(),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token != Token::EOF {
            let stmt = self.parse_statement();
            if let Some(s) = stmt {
                program.statements.push(s);
            }
            self.next_token();
        }

        program
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn get_errors(&self) -> String {
        self.errors
            .iter()
            .fold(format!("{} parser error(s):", self.errors.len()), |a, b| {
                a + "; " + b
            })
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        if !self.expect_peek(&Token::Ident(String::from(""))) {
            return None;
        }

        let ident = match &self.cur_token {
            Token::Ident(ident) => ident.clone(),
            _ => panic!("expected identifier token"),
        };
        let name = Identifier { value: ident };

        if !self.expect_peek(&Token::Assign) {
            return None;
        }

        // TODO: Skipping expression for now
        while self.peek_token != Token::Semicolon {
            self.next_token();
        }

        Some(Box::new(LetStatement { name: name }))
    }

    fn next_token(&mut self) {
        // TODO: Can I avoid the clone without using a swap?
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, expected_token: &Token) -> bool {
        if std::mem::discriminant(&self.peek_token) == std::mem::discriminant(expected_token) {
            self.next_token();
            true
        } else {
            self.errors.push(format!(
                "expected next token to be {:?}, got {:?} instead",
                expected_token, self.peek_token
            ));
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::ast::*;
    use super::super::lexer::Lexer;
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;
                     let y = 10;
                     let foobar = 838383;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);
        if program.statements.len() != 3 {
            panic!(
                "program.statements.len() != 3, got={}",
                program.statements.len()
            );
        }

        let expected_identifiers = vec!["x", "y", "foobar"];
        for (i, ident) in expected_identifiers.into_iter().enumerate() {
            let statement = &program.statements[i];
            test_let_statement(statement, ident);
        }
    }

    fn test_let_statement(statement: &Box<dyn Statement>, name: &str) {
        match statement.as_any().downcast_ref::<LetStatement>() {
            Some(let_statement) => {
                if let_statement.name.value != name {
                    panic!(
                        "wrong identifier name, expected={}, got={}",
                        name, let_statement.name.value
                    );
                }
            }
            None => panic!("statement is not a LetStatement"),
        }
    }

    fn check_parser_errors(parser: &Parser) {
        if parser.has_errors() {
            panic!("{}", parser.get_errors());
        }
    }
}
