#[derive(Debug)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    INSTRUCTION,
    LABEL,
    INTEGER,
    STRING,
    REGISTER,
    METHOD,
    VARIABLE_DECL,
    VARIABLE,
    INCLUDE,
    CHAR
}

pub struct Token {
    pub token_t: TokenType,
    pub data: String
}

impl Token {
    pub fn new(t: TokenType, d: String) -> Self {
        return Token {
            token_t: t,
            data: d
        }
    }
}
