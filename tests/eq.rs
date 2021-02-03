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
fn eq_test() {
    assert_eq!(
        interpretate_string(r#"A is one? ${$a = "1"}"#, &VARIABLES).unwrap(),
        "A is one? true"
    );
}

#[test]
fn two_var_eq() {
    assert_eq!(
        interpretate_string("A == B? ${$a=$b}", &VARIABLES).unwrap(),
        "A == B? false"
    );
}