use std::collections::HashMap;

mod btree;
//use btree::BNode;

fn main() {

    let mut variables: HashMap<String, i32> = HashMap::new();
    
    variables.insert("A".to_string(), 1);
    variables.insert("B".to_string(), 2);

    //let command_example = "A: ${$A}\nB: ${$B==true?10:15}";
    let command_example = "A: ${$A - $B + $B - $A}\nB: ${$B}";
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

    token_parse(&tokenized[..], variables)
}

fn token_parse(tokens: &[Token], variables: &HashMap<String, i32>) -> Result<String, ()> {

    if tokens.len() <= 1 {
        return Ok(match tokens[0] {
            Token::Variable(var_name) => variables.get(var_name).unwrap().to_string(),
            _ => unimplemented!("non variable variant in token parse")
        })
    }

    let (min_operation_index, _) = tokens.iter()
                                            .enumerate().rev()
                                            .filter_map(|(i, t)| if let Token::Operation(o) = t { Some((i, get_operation_priority(*o))) } else { None }) // Get only operation
                                            .min_by(|(_, p_1), (_, p_2)| p_1.cmp(p_2)).unwrap();
    let min_operation = if let Token::Operation(operation) = &tokens[min_operation_index] {
        *operation
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

fn get_operation_priority(operation: Operation) -> u8 {
    match operation {
        Operation::Plus => 2,
        Operation::Minus => 2,
        Operation::Mul => 1,
        _ => unimplemented!("priority of non-math operation")
    }
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
            b'-' => Token::Operation(Operation::Minus),
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

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    Plus,
    Minus,
    Mul,
    TernaryQuestion,
    TernaryElse
}

fn get_code_block(input: &str) -> Option<(usize, usize)> {
    let start = input.chars().position(|c| c == '$')?;
    let end = (&input[start..]).chars().position(|c| c == '}')? + start + 1;

    Some((start, end))
}