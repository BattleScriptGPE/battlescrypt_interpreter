use std::{slice::Iter, sync::Arc};

use lazy_static::__Deref;

use crate::{tokens::{Token, TokenInfo, EOF, EOF_RAW, ILLEGAL}, ast::{AST, PrintStatement}};

pub struct Parser<'a> {
    // On set une période de vie 'a à la struct
    lexer_tokens: Iter<'a, TokenInfo>,
    current_token: TokenInfo, // Les variables avec 'a dureront aussi longtemps que la struct
    next_token: TokenInfo,
    ast_list: Vec<Arc<dyn AST>>
}

impl Parser<'_>  {
    /* pub fn new(init_lexer_tokens: Vec<TokenInfo>) -> Self {
        let lexer_tokens_from_parent: Vec<TokenInfo> = init_lexer_tokens.clone();
        let temp_token_info = TokenInfo::new("None".to_string(), "None".to_string());

        Parser {
            lexer_tokens: lexer_tokens_from_parent.iter(),
            current_token: temp_token_info,
            next_token: temp_token_info,
        }
    } */

    pub fn new<'a>(
        lexer_tokens: Iter<'a, TokenInfo>,
        current_token: TokenInfo,
        next_token: TokenInfo,
        ast_list: Vec<Arc<dyn AST>>

    ) -> Parser<'a> {
        Parser {
            lexer_tokens: lexer_tokens,
            current_token: current_token,
            next_token: next_token,
            ast_list: ast_list
        }
    }

    pub fn get_ast(&self)-> Vec<Arc<dyn AST>> {
        return self.ast_list.clone();
    }

    pub fn run_parsing(&mut self) {
        self.ast_list.push(Arc::new(PrintStatement));
        //TODO self.update();
    }

    fn update(&mut self) {
        println!(
            "curr_tok -> {:?} ; new_token -> {:?}",
            self.current_token, self.next_token
        );
        self.current_token = self.next_token.clone();
        /* println!("curr_tok -> {:?}", self.current_token);
        println!("next_tok -> {:?}", self.next_token); */
        self.next_token = self.next();

        /* let iterate_through_next = self.lexer_tokens.next();
        if iterate_through_next.is_some() {
            self.next_token = *iterate_through_next.unwrap();
            println!("UPDATE UNWRAPED -> {:?}", self.next_token);
        } */
    }

    fn next(&self) -> TokenInfo {
        //let iterate_through_next = self.lexer_tokens.next();

        while self.current_token.0 != EOF {
            if let statement = self.parse_statement() {
                // TODO
                return statement;
            }
        }
        return TokenInfo::new(EOF.to_string(), EOF_RAW.to_string());
    }

    fn parse_statement(&self) -> TokenInfo {
        if self.current_token.0 == ILLEGAL {
            println!("TODO RAISE ERROR");
        }
        println!("TODO parse_statements");
        return TokenInfo::new("PARSE".to_string(), "STATEMENT".to_string()); // TODO
    }
}
