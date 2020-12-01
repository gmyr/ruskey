use std::any::Any;
use std::fmt;

pub trait Node: fmt::Display {}

pub trait Statement: Node {
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl Node for Program {}

pub struct Identifier {
    pub value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Node for Identifier {}

impl Expression for Identifier {}

pub struct LetStatement {
    pub name: Box<Identifier>,
    // pub value: Box<dyn Expression>,
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: add expression
        write!(f, "let {} = ...;", self.name)
    }
}

impl Node for LetStatement {}

impl Statement for LetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ReturnStatement {
    // pub value: Box<dyn Expression>,
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: add expression
        write!(f, "return ...;")
    }
}

impl Node for ReturnStatement {}

impl Statement for ReturnStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct ExpressionStatement {
    // expression: Box<dyn Expression>,
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: add expression
        write!(f, "")
    }
}

impl Node for ExpressionStatement {}

impl Statement for ExpressionStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_display() {
        let program = Program {
            statements: vec![Box::new(LetStatement {
                name: Box::new(Identifier {
                    value: String::from("myVar"),
                }),
            })],
        };

        let a = format!("{}", program);
        // TODO: update once expressions are implemented
        assert_eq!(a, "let myVar = ...;");
    }
}
