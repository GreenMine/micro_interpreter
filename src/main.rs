use std::collections::HashMap;

pub mod interpreter;

fn main() {
    let mut variables: HashMap<String, i32> = HashMap::new();
    
    variables.insert("A".to_string(), 1);
    variables.insert("B".to_string(), 2);
    variables.insert("C".to_string(), 1);

    //let command_example = "A: ${$A - $B+$B-$A}\nB: ${$B + \"string literal\"}";
    let command_example = r#"A: ${$A + $B = $C + $B}\nB: ${$B = "2" ? $A + "123" : $B + "321"}"#;
    let iterpretated_string = interpreter::interpretate_string(command_example, &variables);
    println!("Interpretated result: {:?}", iterpretated_string);
}