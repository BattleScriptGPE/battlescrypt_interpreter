use std::{slice::Iter, sync::Arc};

use lazy_static::__Deref;

use crate::{
    ast::{
        EOFStatement, IdentifierExpression, InfixExpression, LiteralExpression, PrefixExpression,
        PrintStatement, VarStatement, AST, AssignStatement,
    },
    tokens::{get_precedence, Priority, Token, TokenInfo, EOF, EOF_RAW, ILLEGAL},
};

pub struct Parser<'a> {
    // On set une période de vie 'a à la struct
    lexer_tokens: Iter<'a, TokenInfo>,
    current_token: TokenInfo, // Les variables avec 'a dureront aussi longtemps que la struct
    next_token: TokenInfo,
    ast_list: Vec<Arc<dyn AST>>,
}

impl Parser<'_> {
    pub fn new<'a>(
        lexer_tokens: Iter<'a, TokenInfo>,
        current_token: TokenInfo,
        next_token: TokenInfo,
        ast_list: Vec<Arc<dyn AST>>,
    ) -> Parser<'a> {
        Parser {
            lexer_tokens: lexer_tokens,
            current_token: current_token,
            next_token: next_token,
            ast_list: ast_list,
        }
    }

    pub fn get_ast(&self) -> Vec<Arc<dyn AST>> {
        return self.ast_list.clone();
    }

    pub fn run_parsing(&mut self) {
        // As everything is at None, we init first to next() values
        self.update();
        self.update();

        while self.current_token.0 != EOF {

            println!(
                "Managing token -> {:?}, {:?}",
                self.current_token, self.next_token
            );

            let statement = self.parse_statement();

            if statement.is_some() {
                println!("push");
                self.ast_list.push(statement.unwrap());
            }
        }
        self.ast_list.push(Arc::new(EOFStatement));
    }

    fn update(&mut self) {
        /* println!(
            "curr_tok -> {:?} ; new_token -> {:?}",
            self.current_token, self.next_token
        ); */

        self.current_token = self.next_token.clone();
        self.next_token = self.next();
        //println!("update -> {:?}, {:?}", self.current_token, self.next_token);
        /* println!(
            "curr_tok2 -> {:?} ; new_token2 -> {:?}",
            self.current_token, self.next_token
        );*/
    }

    fn next(&mut self) -> TokenInfo {
        let next_token = self.lexer_tokens.next();
        if next_token.is_some() {
            return next_token.unwrap().clone();
        }
        return TokenInfo::new(EOF.to_string(), EOF_RAW.to_string());
    }

    fn parse_statement(&mut self) -> Option<Arc<dyn AST>> {
        if self.current_token.0 == ILLEGAL {
            panic!("ILLEGAL TOKEN");
        }

        println!("TODO parse_statements");

        let mut statement = self.parse_var_statement();

        if statement.is_some() {
            println!("Detected var_statement");
            return statement;
        }

        statement = self.parse_assign_statement();

        if statement.is_some() {
            println!("Detected parse_assign_statement");
            return statement;
        }

        // TODO Move Statement

        statement = self.print_statement();
        

        if statement.is_some() {
            println!("Detected print_statement");
            return statement;
        }

        statement = self.parse_expression_statement();

        if statement.is_some() {
            println!("Detected parse_expression_statement");
            return statement;
        }

        return None;
    }

    fn parse_expression_statement(&mut self) -> Option<Arc<dyn AST>> {
        let expression = self.parse_expression(Priority::LOWEST);
        /* if self.is_valid_token(self.next_token.clone(), Token::SEMI) {
            println!("JUL2");
            self.update();
            self.update();
        } */
        return expression;
    }

    fn parse_assign_statement(&mut self) -> Option<Arc<dyn AST>> {
        if self.is_valid_token(self.current_token.clone(), Token::ID) {
            let variable = self.current_token.1.clone();
            self.is_next(Token::ASSIGN);
            self.update();
            let value = self.parse_expression(Priority::LOWEST);
            self.is_next(Token::SEMI);
            self.update();
            return Some(Arc::new(AssignStatement))
        }
        return None;
    }

    fn parse_var_statement(&mut self) -> Option<Arc<dyn AST>> {
        if self.is_valid_token(self.current_token.clone(), Token::VAR) {

            self.is_next(Token::ID);

            let variable = self.current_token.1.clone();

            self.is_next(Token::ASSIGN);
            self.update();
            let value = self.parse_expression(Priority::LOWEST);
            self.is_next(Token::SEMI);
            self.update();
            return Some(Arc::new(VarStatement {variable: variable, value: value}));
        }
        return None;
    }

    fn print_statement(&mut self) -> Option<Arc<dyn AST>> {
        if self.is_valid_token(self.current_token.clone(), Token::PRINT) {
            println!("JUL1");
            // TODO TRAITEMENT
            let state = self.current_token.1.clone();
            self.is_next(Token::LPAREN);
            self.update();
            let value = "MA VALUE TODO";
            self.is_next(Token::RPAREN);
            self.is_next(Token::SEMI);
            self.update();

            return Some(Arc::new(PrintStatement));
        }
        return None;
    }

    fn parse_expression(&mut self, precedence: Priority) -> Option<Arc<dyn AST>> {
        let mut expression = self.parse_datatypes();

        if expression.is_none() {
            expression = self.parse_identifier();
        }

        if expression.is_none() {
            expression = self.parse_unary();
        }

        if expression.is_none() {
            expression = self.parse_group();
        }

        if expression.is_none() {
            panic!("Parsing expression failed");
        }

        while !self.is_valid_token(self.next_token.clone(), Token::SEMI)
            && precedence <= get_precedence(&self.next_token.0)
        {
            let new_expression = self.parse_infix_expression();
            if new_expression.is_some() {
                expression = new_expression;
            } else {
                break;
            }
        }
        return expression;
    }

    fn parse_datatypes(&mut self) -> Option<Arc<dyn AST>> {
        let data_types = [
            Token::INTEGER.get_name(),
            Token::FLOAT.get_name(),
            Token::TRUE.get_name(),
            Token::FALSE.get_name(),
            Token::STRING.get_name(),
        ];
        if data_types.contains(&&*self.current_token.0) {
            return Some(Arc::new(LiteralExpression));
        }
        return None;
    }

    fn parse_identifier(&self) -> Option<Arc<dyn AST>> {
        if self.is_valid_token(self.current_token.clone(), Token::ID) {
            return Some(Arc::new(IdentifierExpression));
        }
        return None;
    }

    fn parse_unary(&mut self) -> Option<Arc<dyn AST>> {
        let unary_types = [Token::NOT.get_name(), Token::MINUS.get_name()];

        if unary_types.contains(&&*self.current_token.0) {
            let operator = self.current_token.1.clone();
            //let precedence = self.current_token
            // TODO PRECENDENCE ??
            self.update();
            let right = self.parse_expression(Priority::HIGHER);
            // TODO ASSIGNATION PARAMS

            return Some(Arc::new(PrefixExpression));
        } else if self.is_valid_token(self.current_token.clone(), Token::LPAREN) {
            self.update();
            let expression = self.parse_expression(Priority::LOWEST);
            self.is_next(Token::RPAREN);
            return expression;
        }
        return None;
    }

    fn parse_group(&mut self) -> Option<Arc<dyn AST>> {
        if self.is_valid_token(self.current_token.clone(), Token::LPAREN) {
            self.update();
            let expression = self.parse_expression(Priority::LOWEST);
            self.is_next(Token::RPAREN);
            return  expression;
        }
        return None;
    }

    fn parse_infix_expression(&mut self) -> Option<Arc<dyn AST>> {
        let infix_list = [
            // arithmetic operator
            Token::PLUS.get_name(),
            Token::MINUS.get_name(),
            Token::DIV.get_name(),
            Token::MUL.get_name(),
            Token::MODULO.get_name(),
            Token::XOR.get_name(),
            // comparison operator
            Token::EQUAL.get_name(),
            Token::SUPERIOR.get_name(),
            Token::SUPERIOREQ.get_name(),
            Token::INFERIOR.get_name(),
            Token::INFERIOREQ.get_name(),
            Token::NOTEQ.get_name(),
            // logical operator
            Token::AND.get_name(),
            Token::OR.get_name(),
        ];

        if infix_list.contains(&&*self.next_token.0) {
            self.update();
            let operator = self.current_token.1.clone();
            let precedence = get_precedence(&self.current_token.0);
            self.update();
            let right = self.parse_expression(precedence);
            // TODO PARAM BINDING AST
            return Some(Arc::new(InfixExpression));
        }
        return None;
    }

    fn is_valid_token(&self, source_token: TokenInfo, comparable_token: Token) -> bool {
        if source_token.0 == comparable_token.get_name() {
            return true;
        }
        return false;
    }

    fn is_next(&mut self, comparable_token: Token) {
        if !self.is_valid_token(self.next_token.clone(), comparable_token) {
            panic!(
                "Got {:?} instead of {:?}",
                comparable_token, self.next_token
            );
        }
        self.update();
    }
}
