use std::fmt;
use crate::token_type::*;
use crate::object::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize
}


impl Token {
    pub fn new( token_type: TokenType, lexeme: String,  literal: Option<Object>, line: usize) -> Token {
        Token {token_type, lexeme, literal, line}
    }

    pub fn eof(line: usize) -> Token {
        Token{
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result{
        write!(f, "{:?} {} {} \n",
            self.token_type, self.lexeme,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        )
    }

}