use std::collections::HashMap;
use super::ParseResult;

pub fn interpretate_string(mut input: &str, variables: &HashMap<String, i32>) -> ParseResult {
    let mut result = String::with_capacity(input.len());
    while let Some((start, end)) = get_code_block(input) {
        result += &input[..start];

        //Code block parsing
        //println!("In code block expr: {}", &input[start..end]);

        result += &super::interpretate_expr(&input[start + 1..end - 1], variables)?[..];

        input = &input[end..];
    }
    result += &input[..];

    Ok(result)
}

fn get_code_block(input: &str) -> Option<(usize, usize)> {
    let start = input.chars().position(|c| c == '{')?;
    let end = (&input[start..]).chars().position(|c| c == '}')? + start + 1;

    Some((start, end))
}