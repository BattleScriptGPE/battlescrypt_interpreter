use named_tuple::named_tuple;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

named_tuple!(
    #[derive(Debug)]
    pub struct TokenInfo {
        // 'a because we don't know lifetime for vars
        name: String,
        value: String,
    }
);

#[derive(Debug, EnumIter)]
pub enum Token {
    PLUS,
    MINUS,
    MUL,
    DIV,
    STRING,
}

impl Token {
    pub fn to_string(&self) -> &'static str {
        match self {
            Token::PLUS => r#"\+"#,
            Token::MINUS => r#"\-"#,
            Token::MUL => r#"\*"#,
            Token::DIV => r#"/"#,
            Token::STRING => r#"(".*")|('.*')"#,
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Token::PLUS => "PLUS",
            Token::MINUS => "MINUS",
            Token::MUL => "MUL",
            Token::DIV => "DIV",
            Token::STRING => "STRING",
        }
    }
}

// TODO !!! Increments Token number as we add values !!!
pub static TOKEN_ITERATOR: [Token; 5] = [
    Token::PLUS,
    Token::MINUS,
    Token::MUL,
    Token::DIV,
    Token::STRING,
];

pub const EOF: &str = "EOF";
pub const ILLEGAL: &str = "ILLEGAL";
