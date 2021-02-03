use super::types::{Token, Operation};
use std::collections::HashMap;

pub fn parse_expr(expr: &str, variables: &HashMap<String, i32>) -> Result<String, ()> {
    let tokenized = super::tokenizer::tokenize(expr);

    token_parse(&tokenized[..], variables)
}

pub fn token_parse(tokens: &[Token], variables: &HashMap<String, i32>) -> Result<String, ()> {
    //Base case
    if tokens.len() <= 1 {
        return Ok(match tokens[0] {
            Token::Variable(var_name) => variables.get(var_name).unwrap().to_string(),
            _ => unimplemented!("non variable variant in token parse")
        })
    }

    let (min_operation_index, _) = tokens.iter()
                                            .enumerate().rev()
                                            .filter_map(|(i, t)| if let Token::Operation(o) = t { Some((i, super::types::get_operation_priority(*o))) } else { None }) // Get only operation
                                            .min_by(|(_, p_1), (_, p_2)| p_1.cmp(p_2)).unwrap();
    let min_operation = if let Token::Operation(operation) = tokens[min_operation_index] {
        operation
    } else {
        unreachable!()
    };

    println!("Min of operation: {:?} on {} position.", min_operation, min_operation_index);

    let (left_half, right_half) = (
        token_parse(&tokens[..min_operation_index], variables)?,
        token_parse(&tokens[min_operation_index + 1..], variables)?,
    );

    return Ok(match min_operation {
        Operation::Plus => left_half + &right_half[..],
        Operation::Minus => left_half + "-" + &right_half[..],
        _ => unimplemented!("non-math operation in token parse")
    });
}