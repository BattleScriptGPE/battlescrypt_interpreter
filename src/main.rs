use std::fs::File;
use std::io::Read;
use std::slice::Iter;
use std::sync::Arc;

use log::info;

mod ast;
mod lexer;
mod parser;
mod tokens;

use crate::ast::AST;
use crate::lexer::lexer;

use crate::parser::Parser;
use crate::tokens::{TokenInfo, NONE};

fn main() {
    info!("Entering RUST interpreter.");

    let file_path: String = String::from("./stash.amn");

    println!("File Path -> {}", file_path);

    let file_content: String = read_file_from_path(file_path);

    println!("Data retrieved -> \n{}", file_content);

    interpreter(file_content);
}

fn read_file_from_path(path: String) -> String {
    let mut file: File = File::open(path).expect("File not found");

    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    return data;
}

fn interpreter(file_content: String) {
    let lexer_tokens: Vec<TokenInfo> = lexer(file_content);

    println!("TOKENS FINAUX -> {:?}", lexer_tokens);

    let lexer_tokens_iterator: Iter<TokenInfo> = lexer_tokens.iter();

    let current_token: TokenInfo = TokenInfo::new(NONE.to_string(), NONE.to_string());
    let next_token: TokenInfo = TokenInfo::new(NONE.to_string(), NONE.to_string());

    let mut ast_result_list: Vec<Arc<dyn AST>> = Vec::new();

    println!("ast_result_list -> {:?}:?", ast_result_list);

    let mut parser = Parser::new(
        lexer_tokens_iterator,
        current_token,
        next_token,
        ast_result_list,
    );

    parser.run_parsing();
    
    println!("PARSING FINI");

    ast_result_list = parser.get_ast();

    println!("ast_result_list -> {:#?}:?", ast_result_list);

    for branch in ast_result_list {
        branch.evaluate();
    }
}
