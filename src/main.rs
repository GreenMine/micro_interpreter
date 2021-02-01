use std::collections::HashMap;

fn main() {
    let mut variables: HashMap<String, i32> = HashMap::new();
    
    variables.insert("A".to_string(), 1);
    variables.insert("B".to_string(), 2);


    let command_example = "A: ${$A}\nB: ${$B == true ? 10 : 15}";
    println!("Interpretated result: {:?}", interpretate_string(command_example, &mut variables));
}

fn interpretate_string(mut input: &str, _variables: &mut HashMap<String, i32>) -> String {

    let mut result = String::with_capacity(input.len());
    while let Some((start, end)) = get_code_block(input) {
        result += &input[..start];

        //Code block parsing
        let current_expr = &input[start..end];
        println!("In code block expr: {}", current_expr);

        result += "`Here must be expr result`";

        input = &input[end..];
    }
    result += &input[..];

    result
}

fn get_code_block(expr: &str) -> Option<(usize, usize)> {
    let start = expr.chars().position(|c| c == '$')?;
    let end = (&expr[start..]).chars().position(|c| c == '}')? + start + 1;

    Some((start, end))
}