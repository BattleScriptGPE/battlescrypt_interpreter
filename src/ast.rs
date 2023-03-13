use std::{any::Any, path::Prefix, sync::Arc};

use strum_macros::Display;
use substring::Substring;

use crate::tokens::Token;

// TODOOOOOOOOOOOOOOOOOOOOOO NONE EN OPTION NONE

pub trait AST: std::fmt::Debug {
    fn new(&self); // Supprimer ??
    fn to_string(&self);
    fn evaluate(&self) -> Option<ASTTypes>;
    fn get_ast_version(&self) -> ASTVersion;
}

pub enum ASTVersion {
    STATEMENT,
    EXPRESSION,
}


#[derive(Debug, Clone)]
pub struct PrintStatement {
    pub state: String,
    pub value: Option<Arc<dyn AST>>,
}

#[derive(Debug, Clone, Copy)]
pub struct EOFStatement;

#[derive(Debug, Clone)]
pub struct VarStatement {
    pub variable: String,
    pub value: Option<Arc<dyn AST>>,
}

#[derive(Debug, Clone)]
pub struct AssignStatement {
    pub variable: String,
    pub value: Option<Arc<dyn AST>>,
}

impl AST for PrintStatement {
    fn new(&self) {
        println!("new Print");
    }

    fn to_string(&self) {
        println!("to_string Print");
    }

    fn evaluate(&self) -> Option<ASTTypes> {
        //println!("evaluate Print");
        if self.value.is_none() {
            panic!("Print value null");
        }
        let unwrapped_value = self.value.clone().unwrap();
        let result_option = unwrapped_value.evaluate();
        if result_option == None {
            panic!("PROBLEM");
        }
        let mut result = result_option.unwrap().get_value();

        if result.contains(r"\n") {
            result = result.replace(r"\n", "\n");
        }
        println!("{}", result);
        return None;
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::STATEMENT;
    }
}

impl AST for EOFStatement {
    fn new(&self) {
        println!("new EOF");
    }

    fn to_string(&self) {
        println!("to_string EOF");
    }

    fn evaluate(&self) -> Option<ASTTypes> {
        //println!("evaluate EOF");
        return None;
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

    fn evaluate(&self) -> Option<ASTTypes> {
        println!("evaluate VAR");
        return None;
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

    fn evaluate(&self) -> Option<ASTTypes> {
        println!("evaluate ASSIGN");
        return None;
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::STATEMENT;
    }
}

#[derive(Debug, Clone)]
pub struct LiteralExpression {
    pub type_literal: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct IdentifierExpression {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub variable: String,
    pub value: Option<Arc<dyn AST>>,
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub left: Option<Arc<dyn AST>>,
    pub right: Option<Arc<dyn AST>>,
    pub operator: String,
}

#[derive(PartialEq, Display, Debug)]
pub enum ASTTypes {
    INTEGER(i64),
    FLOAT(f64),
    STRING(String),
    BOOLEAN(bool),
}

impl ASTTypes {
    pub fn get_value(&self) -> String {
        match self {
            ASTTypes::INTEGER(a) => a.clone().to_string(),
            ASTTypes::FLOAT(a) => a.clone().to_string(),
            ASTTypes::STRING(a) => a.clone(),
            ASTTypes::BOOLEAN(a) => a.to_string().clone(),
        }
    }
}

impl AST for LiteralExpression {
    fn new(&self) {
        println!("new VAR");
    }

    fn to_string(&self) {
        println!("to_string VAR");
    }

    fn evaluate(&self) -> Option<ASTTypes> {
        println!("evaluate VAR");
        let out = self.type_casting();

        if out == None {
            panic!("type_casting NONE value");
        }
        return out;
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::EXPRESSION;
    }
}

impl LiteralExpression {
    pub fn type_casting(&self) -> Option<ASTTypes> {
        if self.type_literal == Token::INTEGER.get_name() {
            return Some(ASTTypes::INTEGER(self.value.parse::<i64>().unwrap()));
        } else if self.type_literal == Token::FLOAT.get_name() {
            return Some(ASTTypes::FLOAT(self.value.parse::<f64>().unwrap()));
        } else if self.type_literal == Token::STRING.get_name() {
            return Some(ASTTypes::STRING(LiteralExpression::strip_quotes_from_string(self.value.to_string())));
        } else if self.type_literal == Token::TRUE.get_name() {
            return Some(ASTTypes::BOOLEAN(true));
        } else if self.type_literal == Token::FALSE.get_name() {
            return Some(ASTTypes::BOOLEAN(false));
        }
        return None;
    }

    pub fn strip_quotes_from_string(expression: String) -> String {
        let first = expression.chars().nth(0);
        let last = expression.chars().nth(expression.len() - 1);
        if first.is_none() || last.is_none() || first.unwrap() != '"' || last.unwrap() != '"' {
            panic!("Can't strip quotes from string");
        }
        return expression.substring(1, expression.len() - 1).to_string();
    }
}

impl AST for IdentifierExpression {
    fn new(&self) {
        println!("new VAR");
    }

    fn to_string(&self) {
        println!("to_string VAR");
    }

    fn evaluate(&self) -> Option<ASTTypes> {
        println!("evaluate VAR");
        return None;
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

    fn evaluate(&self) -> Option<ASTTypes> {
        println!("evaluate VAR");
        return None;
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

    fn evaluate(&self) -> Option<ASTTypes> {
        println!("evaluate VAR");
        return None;
    }

    fn get_ast_version(&self) -> ASTVersion {
        return ASTVersion::EXPRESSION;
    }
}
