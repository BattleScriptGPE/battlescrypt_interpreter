use std::path::Prefix;

pub trait AST: std::fmt::Debug {
    fn new(&self);
    fn to_string(&self);
    fn evaluate(&self);
    fn get_ast_version(&self) -> ASTVersion;
}

pub enum ASTVersion {
    STATEMENT,
    EXPRESSION,
}

/* pub trait Statement: AST {}

pub trait Expression: AST {} */

#[derive(Debug, Clone, Copy)]
pub struct PrintStatement;

#[derive(Debug, Clone, Copy)]
pub struct EOFStatement;

#[derive(Debug, Clone, Copy)]
pub struct VarStatement;

impl AST for PrintStatement {
    fn new(&self) {
        println!("new Print");
    }

    fn to_string(&self) {
        println!("to_string Print");
    }

    fn evaluate(&self) {
        println!("evaluate Print");
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::STATEMENT;
    }
}

//impl Statement for PrintStatement {}

impl AST for EOFStatement {
    fn new(&self) {
        println!("new EOF");
    }

    fn to_string(&self) {
        println!("to_string EOF");
    }

    fn evaluate(&self) {
        println!("evaluate EOF");
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::STATEMENT;
    }
}

impl AST for VarStatement {
    fn new(&self) {
        println!("new VAR");
    }

    fn to_string(&self) {
        println!("to_string VAR");
    }

    fn evaluate(&self) {
        println!("evaluate VAR");
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::STATEMENT;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LiteralExpression;

#[derive(Debug, Clone, Copy)]
pub struct IdentifierExpression;

#[derive(Debug, Clone, Copy)]
pub struct PrefixExpression;

#[derive(Debug, Clone, Copy)]
pub struct InfixExpression;

impl AST for LiteralExpression {
    fn new(&self) {
        println!("new VAR");
    }

    fn to_string(&self) {
        println!("to_string VAR");
    }

    fn evaluate(&self) {
        println!("evaluate VAR");
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::EXPRESSION;
    }
}

impl AST for IdentifierExpression {
    fn new(&self) {
        println!("new VAR");
    }

    fn to_string(&self) {
        println!("to_string VAR");
    }

    fn evaluate(&self) {
        println!("evaluate VAR");
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::EXPRESSION;
    }
}

impl AST for PrefixExpression {
    fn new(&self) {
        println!("new VAR");
    }

    fn to_string(&self) {
        println!("to_string VAR");
    }

    fn evaluate(&self) {
        println!("evaluate VAR");
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::EXPRESSION;
    }
}

impl AST for InfixExpression {
    fn new(&self) {
        println!("new VAR");
    }

    fn to_string(&self) {
        println!("to_string VAR");
    }

    fn evaluate(&self) {
        println!("evaluate VAR");
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::EXPRESSION;
    }
}

/* struct Cat;

impl Animal for Cat {
    fn tell(&self) { println!("Meow"); }
    fn pet(&mut self) { println!("purr");
    fn feed(&mut self, food: Food) { println!("lick"); }
} */
