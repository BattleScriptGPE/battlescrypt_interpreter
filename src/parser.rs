use std::{slice::Iter, sync::Arc};

use lazy_static::__Deref;

use crate::{
    ast::{EOFStatement, PrintStatement, VarStatement, AST},
    tokens::{Token, TokenInfo, EOF, EOF_RAW, ILLEGAL},
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

        while self.current_token.0 != EOF {
            self.update();
            println!("not EOF -> {:?}, {:?}", self.current_token, self.next_token);
            if let statement = self.parse_statement() {
                // TODO
                self.ast_list.push(statement);
            }
        }
        self.ast_list.push(Arc::new(EOFStatement));
    }

    fn update(&mut self) {
        println!(
            "curr_tok -> {:?} ; new_token -> {:?}",
            self.current_token, self.next_token
        );
        self.current_token = self.next_token.clone();
        self.next_token = self.next();
        println!(
            "curr_tok2 -> {:?} ; new_token2 -> {:?}",
            self.current_token, self.next_token
        );
    }

    fn next(&mut self) -> TokenInfo {
        let next_token = self.lexer_tokens.next();
        if next_token.is_some() {
            return next_token.unwrap().clone();
        }
        return TokenInfo::new(EOF.to_string(), EOF_RAW.to_string());
    }

    fn parse_statement(&mut self) -> Arc<dyn AST> {
        if self.current_token.0 == ILLEGAL {
            panic!("ILLEGAL TOKEN");
        }

        println!("TODO parse_statements");

        let parse_var_statement = self.parse_var_statement();

        if parse_var_statement.is_some() {
            return parse_var_statement.unwrap();
        }

        // TODO Parse assign statement

        // TODO Move Statement

        let print_statement = self.print_statement();

        if print_statement.is_some() {
            return print_statement.unwrap();
        }

        // TODO Parse Expression Statement

        return Arc::new(EOFStatement); // TODO
    }

    fn parse_var_statement(&self) -> Option<Arc<dyn AST>> {
        if self.is_valid_token(self.current_token.clone(), Token::VAR) {
            // TODO TRAITEMENT
            return Some(Arc::new(VarStatement));
        }
        return None;
    }

    fn print_statement(&mut self) -> Option<Arc<dyn AST>> {
        if self.is_valid_token(self.current_token.clone(), Token::PRINT) {
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
