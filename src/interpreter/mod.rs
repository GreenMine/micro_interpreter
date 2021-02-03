mod interpreter;
mod resolver;
pub(super) mod tokenizer;
pub mod types;

use resolver::parse_expr;
pub use interpreter::interpretate_string;