use std::fs::File;
use std::io::Read;

use log::info;
use regex::Match;

mod lexer;
mod tokens;

use crate::lexer::lexer;

use crate::tokens::Token;
use crate::tokens::EOF;
use crate::tokens::ILLEGAL;

use crate::tokens::TokenInfo;
use crate::tokens::TOKEN_ITERATOR;

use regex::Regex;

use lazy_static::lazy_static;

fn main() {
    info!("Entering RUST interpreter.");

    let file_path: String = String::from("./stash.amn");

    println!("File Path -> {}", file_path);

    let fileContent: String = read_file_from_path(file_path);

    println!("Data retrieved -> \n{}", fileContent);

    interpreter(fileContent);
}

fn read_file_from_path(path: String) -> String {
    let mut file: File = File::open(path).expect("File not found");

    let mut data: String = String::new();

    file.read_to_string(&mut data)
        .expect("Error while reading file");

    return data;
}

fn interpreter(file_content: String) {
    let lexerTokens: Vec<TokenInfo> = lexer(file_content);

    println!("TOKENS FINAUX -> {:?}", lexerTokens);
}
