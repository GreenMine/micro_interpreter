use super::{types::{Token, Operation}, consts::STRING_LITERAL_SYMBOL};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: &[u8] = input.as_bytes();
    let mut i: usize = 0;

    while i < chars.len() { //Works only with latin symbols
        let token = match chars[i] {
            b'$' => { //Get variable name
                let start_index = i + 1;
                let mut var_index = start_index;
                while var_index < chars.len() && is_variable_char(chars[var_index]) {
                    var_index += 1;
                }
                i = var_index - 1;

                Token::Variable(&input[start_index..var_index])
            },
            STRING_LITERAL_SYMBOL => {//Get string literal
                let start_index = i + 1;
                let mut lit_index = start_index;
                while lit_index < chars.len() && chars[lit_index] != STRING_LITERAL_SYMBOL {
                    lit_index += 1;
                }
                i = lit_index;

                Token::StrLiteral(&input[start_index..i])
            }
            b'(' => Token::StartBracket,
            b')' => Token::EndBracket,
            b'?' => Token::Operation(Operation::TernaryQuestion),
            b':' => Token::Operation(Operation::TernaryElse),
            b'+' => Token::Operation(Operation::Plus),
            b'-' => Token::Operation(Operation::Minus),
            b'=' => Token::Operation(Operation::Eq),
            b'!' => {
                i += 1;
                if chars[i] == b'=' {
                    Token::Operation(Operation::NotEq)
                } else {
                   continue;
                }
            }
            _ => {
                   i += 1;
                   continue;
            } 
        };
        tokens.push(token);

        i += 1;
    }

    tokens
}

fn is_variable_char(symbol: u8) -> bool {
    symbol.is_ascii_alphabetic() || symbol == b'_'
}