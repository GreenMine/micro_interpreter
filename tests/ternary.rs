#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use micro_interpreter::interpreter::interpretate_string;

lazy_static! {
    static ref VARIABLES: HashMap<String, i32> = {
        let mut v = HashMap::new();

        v.insert("a".to_string(), 1);
        v.insert("b".to_string(), 2);

        v
    };
}

#[test]
fn simple_ternary() {
    assert_eq!(
        interpretate_string(r#"A is ${$a = "1" ? "one" : "not one"}!"#, &VARIABLES).unwrap(),
        "A is one!"
    );
}

#[test]
fn nested_ternary() {
    assert_eq!(
        interpretate_string(r#"Result: ${$a = "1" ?
                                            "o" + ($b = "2" ? "t" : $b) :
                                            "a is not one"}"#, &VARIABLES).unwrap(),
        "Result: ot"
    );
}