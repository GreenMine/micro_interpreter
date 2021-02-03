use std::collections::HashMap;

pub fn interpretate_string(mut input: &str, variables: &HashMap<String, i32>) -> Result<String, ()> {
    let mut result = String::with_capacity(input.len());
    while let Some((start, end)) = get_code_block(input) {
        result += &input[..start];

        //Code block parsing
        let current_expr = &input[start..end];
        println!("In code block expr: {}", current_expr);

        result += &super::parse_expr(&input[start + 2..end - 1], variables)?[..];

        input = &input[end..];
    }
    result += &input[..];

    Ok(result)
}

fn get_code_block(input: &str) -> Option<(usize, usize)> {
    let start = input.chars().position(|c| c == '$')?;
    let end = (&input[start..]).chars().position(|c| c == '}')? + start + 1;

    Some((start, end))
}