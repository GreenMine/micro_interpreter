use std::collections::HashMap;

pub mod interpreter;

fn main() {
    let mut variables: HashMap<String, i32> = HashMap::new();
    
    variables.insert("A".to_string(), 1);
    variables.insert("B".to_string(), 2);
    variables.insert("C".to_string(), 3);

    //let command_example = "A: ${$A - $B+$B-$A}\nB: ${$B + \"string literal\"}";
    let command_example = r#"A: ${$A + $B = $C + $B}\nB: ${$B = "2" ? $A + "123" : $B + "321"}\nC: ${$B + ($A + $C) = $B + $A + $C}
    Nested ternary: ${
        $B = "2" ?
        ($A = "3" ? "b = 2 && a = 3" : "b = 2") :
        "???"
    }"#;

    let start = std::time::Instant::now();
    let iterpretated_string = interpreter::interpretate_string(command_example, &variables);
    println!("Elepsed {}micros. to interpretate string.", start.elapsed().as_micros());

    println!("Interpretated result: {:?}", iterpretated_string);
}