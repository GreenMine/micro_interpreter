#[derive(Debug)]
pub enum Error {
    VariableNotFound(String),
    Syntax
}

impl ToString for Error {
    fn to_string(&self) -> String {
        format!("Error: {}!", match self {
                Error::VariableNotFound(var_name) => format!("variable {} not found", var_name),
                Error::Syntax => "syntax error".to_string()
        })
    }
}