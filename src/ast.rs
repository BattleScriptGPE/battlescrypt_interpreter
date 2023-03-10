use std::{path::Prefix, sync::Arc};

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

#[derive(Debug, Clone)]
pub struct PrintStatement {
    pub state: String,
    pub value: Option<Arc<dyn AST>>
}

#[derive(Debug, Clone, Copy)]
pub struct EOFStatement;

#[derive(Debug, Clone)]
pub struct VarStatement {
    pub variable: String,
    pub value: Option<Arc<dyn AST>>
}

#[derive(Debug, Clone)]
pub struct AssignStatement {
    pub variable: String,
    pub value: Option<Arc<dyn AST>>
}

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

impl AST for AssignStatement {
    fn new(&self) {
        println!("new ASSIGN");
    }

    fn to_string(&self) {
        println!("to_string ASSIGN");
    }

    fn evaluate(&self) {
        println!("evaluate ASSIGN");
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::STATEMENT;
    }
}

#[derive(Debug, Clone)]
pub struct LiteralExpression {
    pub type_literal: String,
    pub value: String
}

#[derive(Debug, Clone)]
pub struct IdentifierExpression {
    pub value: String
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub variable: String,
    pub value: Option<Arc<dyn AST>>
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub left: Option<Arc<dyn AST>>,
    pub right: Option<Arc<dyn AST>>,
    pub operator: String
}

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