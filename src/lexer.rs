use regex::Regex;

use crate::tokens::{TokenInfo, EOF, ILLEGAL, TOKEN_ITERATOR};

pub fn lexer(file_content: String) -> Vec<TokenInfo> {
    let mut position = 0;
    let mut detected_token_infos: Vec<TokenInfo> = Vec::new();

    while position < file_content.len() {
        println!("position -> {}", position);

        let mut found = false;

        for token_id in &TOKEN_ITERATOR {
            let re = Regex::new(token_id.get_value()).unwrap();

            if re.is_match(&file_content) {
                let mat = re.find(&file_content).unwrap();
                let match_start = mat.start();
                let match_end = mat.end();

                if match_start == position {
                    position = match_end;
                    println!(
                        "Found is_match VALID, pattern -> {}, start -> {}; end -> {}; value -> {}",
                        token_id.get_value(),
                        match_start,
                        match_end,
                        mat.as_str()
                    );

                    found = true;
                    detected_token_infos.push(TokenInfo::new(
                        token_id.get_name().to_string(),
                        mat.as_str().to_string(),
                    ));
                    println!(
                        "STRUCT INFOS -> {}",
                        detected_token_infos[detected_token_infos.len() - 1].0
                    );
                    println!(
                        "STRUCT INFOS -> {}",
                        detected_token_infos[detected_token_infos.len() - 1].1
                    );
                    break;
                }
            }
        }

        if !found {
            detected_token_infos.push(TokenInfo::new(
                ILLEGAL.to_string(),
                file_content.chars().nth(position).unwrap().to_string(),
            ));
            position += 1;
        }

        println!("{:?}", detected_token_infos);
    }

    detected_token_infos.push(TokenInfo::new(EOF.to_string(), "\x00".to_string()));
    detected_token_infos.push(TokenInfo::new(EOF.to_string(), "\x00".to_string()));

    return detected_token_infos;
}
