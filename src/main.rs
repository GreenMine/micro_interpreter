use std::collections::HashMap;

mod interpreter;

fn main() {
    let mut variables: HashMap<String, i32> = HashMap::new();
    
    variables.insert("A".to_string(), 1);
    variables.insert("B".to_string(), 2);

    //let command_example = "A: ${$A}\nB: ${$B==true?10:15}";
    let command_example = "A: ${$A - $B+$B-$A}\nB: ${$B + \"string literal\"}";
    println!("Interpretated result: {:?}", interpreter::interpretate_string(command_example, &variables));
}