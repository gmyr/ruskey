use std::any::Any;

pub trait Node {}

pub trait Statement: Node {
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {}

pub struct Identifier {
    pub value: String,
}

impl Node for Identifier {}

impl Expression for Identifier {}

pub struct LetStatement {
    pub name: Box<Identifier>,
    // pub value: Box<dyn Expression>,
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

impl Node for ReturnStatement {}

impl Statement for ReturnStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
