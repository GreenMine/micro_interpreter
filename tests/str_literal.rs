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
fn str_literal() {
    assert_eq!(
        interpretate_string(r#"a: {"some a - " + $a}"#, &VARIABLES).unwrap(),
        "a: some a - 1"
    );
}
#[test]
fn two_literal() {
    assert_eq!(
        interpretate_string(r#"result: ({"a and b: " + $a + " and " + $b + "."})"#, &VARIABLES).unwrap(),
        interpretate_string(r#"result: (a and b: {$a} and {$b}.)"#, &VARIABLES).unwrap(),
    );
}