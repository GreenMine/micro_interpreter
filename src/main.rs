use std::collections::HashMap;

mod btree;
//use btree::BNode;

fn main() {

    let mut variables: HashMap<String, i32> = HashMap::new();
    
    variables.insert("A".to_string(), 1);
    variables.insert("B".to_string(), 2);


    //let command_example = "A: ${$A}\nB: ${$B==true?10:15}";
    let command_example = "A: ${$A}\nB: ${$B ? 1 : 2}";
    println!("Interpretated result: {:?}", interpretate_string(command_example, &mut variables));
}

fn interpretate_string(mut input: &str, variables: &mut HashMap<String, i32>) -> Result<String, ()> {
    let mut result = String::with_capacity(input.len());
    while let Some((start, end)) = get_code_block(input) {
        result += &input[..start];

        //Code block parsing
        let current_expr = &input[start..end];
        println!("In code block expr: {}", current_expr);

        result += &parse_expr(&input[start + 2..end - 1],  variables)?[..];

        input = &input[end..];
    }
    result += &input[..];

    Ok(result)
}

fn parse_expr(expr: &str, variables: &mut HashMap<String, i32>) -> Result<String, ()> {
    let tokenized = tokenize(expr);

    if let Ok(res) = token_parse(&tokenized[..], variables) {
        return Ok(res);
    }

    if let Some(pos) = tokenized.iter()
                                      .position(|t| *t == Token::Operation(Operation::TernaryQuestion)) {
       let left_side_result = {
           println!("Left token result: {:?}", token_parse(&tokenized[..pos], variables));
       };
    }

    unimplemented!()
}

fn token_parse(tokens: &[Token], variables: &mut HashMap<String, i32>) -> Result<String, ()> {
    println!("Tokens len: {}", tokens.len());
    if tokens.len() == 0 {
        return Err(());
    }
    if tokens.len() == 1 {
        if let Token::Variable(var) = tokens[0] {
            return Ok(variables[var].to_string());
        } else {
            return Err(());
        }
    }

    Err(())
}

fn tokenize(input: &str) -> Vec<Token> {
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
            _ => {continue;}
        };

        tokens.push(token);
    }

    tokens
}

fn is_variable_name(symbol: char) -> bool {
    symbol.is_alphabetic()
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    StrLiteral(&'a str),
    Variable(&'a str),
    Operation(Operation),
    StartBracket,
    EndBracket
}

#[derive(Debug, PartialEq)]
enum Operation {
    Plus,
    TernaryQuestion,
    TernaryElse
}

fn get_code_block(input: &str) -> Option<(usize, usize)> {
    let start = input.chars().position(|c| c == '$')?;
    let end = (&input[start..]).chars().position(|c| c == '}')? + start + 1;

    Some((start, end))
}