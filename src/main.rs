use std::collections::HashMap;

pub mod interpreter;

fn main() {
    let mut variables: HashMap<String, i32> = HashMap::new();
    
    variables.insert("A".to_string(), 1);
    variables.insert("B".to_string(), 2);
    variables.insert("C".to_string(), 1);

    //let command_example = "A: ${$A - $B+$B-$A}\nB: ${$B + \"string literal\"}";
    let command_example = "A: ${$A + $B = $C + $B}\nB: ${$B}";
    println!("Interpretated result: {:?}", interpreter::interpretate_string(command_example, &variables));

}