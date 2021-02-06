use super::{Error, ParseResult, types::{Token, Operation}};
use std::collections::HashMap;


pub fn interpretate_expr<T: ToString>(expr: &str, variables: &HashMap<String, T>) -> ParseResult {
    let tokenized = super::tokenizer::tokenize(expr);

    token_parse(&tokenized[..], variables)
}

pub fn token_parse<T: ToString>(mut  tokens: &[Token], variables: &HashMap<String, T>) -> ParseResult {
    //Base case
    if tokens.len() <= 1 {
        return Ok(match tokens[0] {
            Token::Variable(var_name) =>  {
                match variables.get(var_name) {
                    Some(var_value) => var_value.to_string(),
                    None => return Err(Error::VariableNotFound(var_name.to_string()))
                }
            },//TODO: Fix always allocate
            Token::StrLiteral(str) => str.to_string(),//TODO: Fix always allocate
            _ => unimplemented!("non variable/const variant in token parse(`{:?}`)", tokens[0])
        })
    }

    if tokens[0] == Token::StartBracket {
        tokens = &tokens[1..tokens.len() - 1];
    }
    //println!("Tokens: {:?}", tokens);
    
    let min_operation_index = get_min_operation_index(tokens);
    let min_operation = if let Token::Operation(operation) = tokens[min_operation_index] {
        operation
    } else {
        unreachable!()
    };

    //println!("Min of operation: {:?} on {} position.", min_operation, min_operation_index);

    let left_half = token_parse(&tokens[..min_operation_index], variables)?;
    let right_half = token_parse(if min_operation == Operation::TernaryQuestion {
        let right_tokens = &tokens[min_operation_index + 1..];
        let else_position = get_min_operation_index(right_tokens);
        if left_half == "true" {
                    &right_tokens[..else_position]
                } else {
                    &right_tokens[else_position + 1..]
                }
    } else {
        &tokens[min_operation_index + 1..]
    }, variables)?;

    return Ok(match min_operation {
        Operation::Plus => left_half + &right_half[..],
        Operation::Minus => left_half + "-" + &right_half[..],
        Operation::Eq => if left_half == right_half { "true".to_string() } else { "false".to_string() }
        Operation::NotEq => if left_half != right_half { "true".to_string() } else { "false".to_string() }
        Operation::TernaryQuestion => right_half,
        _ => unimplemented!("non-understand operation `{:?}` in token parse", min_operation)
    });
}


fn get_min_operation_index(tokens: &[Token]) -> usize {
    let mut pc = 0;
    let (min_operation_index, _) = tokens.iter()
                                                .enumerate()
                                                .filter_map(|(i, t)| {
                                                                if *t == Token::EndBracket   { pc -= 1; return None; }
                                                                if *t == Token::StartBracket { pc += 1; return None; }

                                                                if pc != 0 {
                                                                    return None;
                                                                }

                                                                if let Token::Operation(o) = t {
                                                                    Some((i, super::types::get_operation_priority(*o)))
                                                                } else {
                                                                    None
                                                                }
                                                }) // Get operation and skip brackets
                                                .max_by(|(_, p_1), (_, p_2)| p_1.cmp(p_2)).unwrap();
    min_operation_index
}