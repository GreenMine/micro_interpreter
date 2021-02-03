use super::types::{Token, Operation};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: &[u8] = input.as_bytes();
    for mut i in 0..chars.len() { //Works only with latin symbols
        let token = match chars[i] {
            b'$' => {
                let start_index = i + 1;
                let mut var_index = start_index;
                while var_index < chars.len() && chars[var_index].is_ascii_alphabetic() {
                    var_index += 1;
                }
                i = var_index;

                Token::Variable(&input[start_index..i])
            },
            b'(' => Token::StartBracket,
            b')' => Token::EndBracket,
            b'?' => Token::Operation(Operation::TernaryQuestion),
            b':' => Token::Operation(Operation::TernaryElse),
            b'+' => Token::Operation(Operation::Plus),
            b'-' => Token::Operation(Operation::Minus),
            _ => {continue;}
        };

        tokens.push(token);
    }

    tokens
}