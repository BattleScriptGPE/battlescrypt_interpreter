use named_tuple::named_tuple;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

named_tuple!(
    #[derive(Debug, Clone)]
    pub struct TokenInfo {
        // 'a because we don't know lifetime for vars
        name: String,
        value: String,
    }
);

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Token {
    // Calculator tokens
    PLUS,
    MINUS,
    MUL,
    DIV,
    MODULO,
    LPAREN,
    RPAREN,

    // Logic tokens
    AND,
    OR,
    XOR,
    NOT,

    // Syntax tokens
    SEMI,
    COMMA,
    LBRACE,
    RBRACE,
    LSQUARE,
    RSQUARE,

    // Comparisons tokens
    EQUAL,
    NOTEQ,
    INFERIOR,
    SUPERIOR,
    INFERIOREQ,
    SUPERIOREQ,

    ASSIGN,

    // Types
    STRING,
    FLOAT,
    INTEGER,

    TRUE,
    FALSE,

    // Reserved keywords
    VAR,
    FUNCTION,
    IF,
    WHILE,
    ELSE,
    RETURN,
    NAN,
    PRINT,
    WHITESPACE,
    COMMENT,

    ID,
}

impl Token {
    pub fn get_value(&self) -> &'static str {
        match self {
            Token::PLUS => r#"\+"#,
            Token::MINUS => r#"\-"#,
            Token::MUL => r#"\*"#,
            Token::DIV => r#"/"#,
            Token::MODULO => r#"%"#,
            Token::LPAREN => r#"\("#,
            Token::RPAREN => r#"\)"#,

            Token::AND => r#"&&"#,
            Token::OR => r#"\|\|"#,
            Token::XOR => r#"\^"#,
            Token::NOT => r#"!"#,

            Token::SEMI => r#";"#,
            Token::COMMA => r#","#,
            Token::LBRACE => r#"\{"#,
            Token::RBRACE => r#"\}"#,
            Token::LSQUARE => r#"\["#,
            Token::RSQUARE => r#"\]"#,

            Token::EQUAL => r#"=="#,
            Token::NOTEQ => r#"!="#,
            Token::INFERIOR => r#"<"#,
            Token::SUPERIOR => r#">"#,
            Token::INFERIOREQ => r#"<="#,
            Token::SUPERIOREQ => r#">="#,

            Token::ASSIGN => r#"="#,

            Token::STRING => r#"(".*")|('.*')"#,
            Token::FLOAT => r#"\d+\.\d+"#,
            Token::INTEGER => r#"\d+"#,

            Token::TRUE => r#"true"#,
            Token::FALSE => r#"false"#,

            Token::VAR => r#"var"#,
            Token::FUNCTION => r#"function"#,
            Token::IF => r#"if"#,
            Token::WHILE => r#"while"#,
            Token::ELSE => r#"else"#,
            Token::RETURN => r#"return"#,
            Token::NAN => r#"nan"#,
            Token::PRINT => r#"print"#,
            Token::WHITESPACE => r#"(\t|\n|\s|\r)+"#,
            Token::COMMENT => r#"#.*"#,
            Token::ID => r#"[_a-zA-Z][_a-zA-Z0-9]*"#,
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Token::PLUS => "PLUS",
            Token::MINUS => "MINUS",
            Token::MUL => "MUL",
            Token::DIV => "DIV",
            Token::MODULO => "MODULO",
            Token::LPAREN => "LPAREN",
            Token::RPAREN => "RPAREN",

            Token::AND => "AND",
            Token::OR => "OR",
            Token::XOR => "XOR",
            Token::NOT => "NOT",

            Token::SEMI => "SEMI",
            Token::COMMA => "COMMA",
            Token::LBRACE => "LBRACE",
            Token::RBRACE => "RBRACE",
            Token::LSQUARE => "LSQUARE",
            Token::RSQUARE => "RSQUARE",

            Token::EQUAL => "EQUAL",
            Token::NOTEQ => "NOTEQ",
            Token::INFERIOR => "INFERIOR",
            Token::SUPERIOR => "SUPERIOR",
            Token::INFERIOREQ => "INFERIOREQ",
            Token::SUPERIOREQ => "SUPERIOREQ",

            Token::ASSIGN => "ASSIGN",

            Token::STRING => "STRING",
            Token::FLOAT => "FLOAT",
            Token::INTEGER => "INTEGER",

            Token::TRUE => "TRUE",
            Token::FALSE => "FALSE",

            Token::VAR => "VAR",
            Token::FUNCTION => "FUNCTION",
            Token::IF => "IF",
            Token::WHILE => "WHILE",
            Token::ELSE => "ELSE",
            Token::RETURN => "RETURN",
            Token::NAN => "NAN",
            Token::PRINT => "PRINT",
            Token::WHITESPACE => "WHITESPACE",
            Token::COMMENT => "COMMENT",

            Token::ID => "ID",
        }
    }
}

// TODO !!! Increments Token number as we add values !!!
pub static TOKEN_ITERATOR: [Token; 40] = [
    Token::PLUS,
    Token::MINUS,
    Token::MUL,
    Token::DIV,
    Token::MODULO,
    Token::LPAREN,
    Token::RPAREN,
    Token::AND,
    Token::OR,
    Token::XOR,
    Token::NOT,
    Token::SEMI,
    Token::COMMA,
    Token::LBRACE,
    Token::RBRACE,
    Token::LSQUARE,
    Token::RSQUARE,
    Token::EQUAL,
    Token::NOTEQ,
    Token::INFERIOR,
    Token::SUPERIOR,
    Token::INFERIOREQ,
    Token::SUPERIOREQ,
    Token::ASSIGN,
    Token::STRING,
    Token::FLOAT,
    Token::INTEGER,
    Token::TRUE,
    Token::FALSE,
    Token::VAR,
    Token::FUNCTION,
    Token::IF,
    Token::WHILE,
    Token::ELSE,
    Token::RETURN,
    Token::NAN,
    Token::PRINT,
    Token::WHITESPACE,
    Token::COMMENT,
    Token::ID,
];

pub const EOF: &str = "EOF";
pub const EOF_RAW: &str = "\x00";

pub const ILLEGAL: &str = "ILLEGAL";

pub const NONE: &str = "NONE";
