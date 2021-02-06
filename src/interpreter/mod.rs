mod interpreter;
mod resolver;
pub(super) mod tokenizer;
pub mod types;
mod error;

pub use resolver::interpretate_expr;
pub use interpreter::interpretate_string;
pub use error::Error;

pub type ParseResult = Result<String, error::Error>;