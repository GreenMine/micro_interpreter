#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use micro_interpreter::interpreter::interpretate_string;

lazy_static! {
    static ref VARIABLES: HashMap<String, i32> = {
        let mut v = HashMap::new();

        v.insert("var_a".to_string(), 1);
        v.insert("var_b".to_string(), 2);

        v
    };
}

#[test]
fn simgle_var() {
    assert_eq!(
        interpretate_string("a: {$var_a}", &VARIABLES).unwrap(),
        "a: 1"
    );
}

#[test]
fn text_after_var() {
    assert_eq!(
        interpretate_string("b: {$var_b}, some text", &VARIABLES).unwrap(),
        "b: 2, some text"
    );
}