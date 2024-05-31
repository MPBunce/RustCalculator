use crate::token::*;
use crate::error::*;
use crate::token_type::TokenType;
use std::collections::HashMap;
use crate::object::*;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let mut keywords = HashMap::new();

        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);

        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords
        }
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError>{
        let mut had_error: Option<LoxError> = None;
        while !self.is_at_end(){
            self.start = self.current;
            match self.scan_token(){
                Ok(_) => {}
                Err(e) => {
                    e.report("");
                    had_error = Some(e);
                }
            }
        }

        self.tokens.push( Token::eof(self.line) );

        if let Some(e) = had_error{
            Err(e)
        } else {
            Ok(&self.tokens)
        }

    }
    fn scan_token(&mut self) -> Result<(), LoxError>{
        let c = self.advance();
        //println!("Char: {:?}", &c);
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let tok = if self.is_match('='){
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                self.add_token(tok);

            },
            '=' => {
                let tok = if self.is_match('='){
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(tok);

            },
            '<' => {
                let tok = if self.is_match('='){
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                self.add_token(tok);

            },
            '>' => {
                let tok = if self.is_match('='){
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                self.add_token(tok);

            },
            '/' => {
                if self.is_match('/'){
                    while self.peek() != '\n' && !self.is_at_end(){
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            },
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => {
                self.line += 1;
            },
            '"' => {
                self.is_string()?;
            },
            'o' => {
                if self.is_match('r'){
                    self.add_token(TokenType::Or)
                }
            }
            _ => {
                if c.is_digit(10){
                    self.is_number()?;
                } else if c.is_ascii_alphabetic() || c == '_'{
                    println!("{}", c);
                    self.identifier()
                } else {
                    return Err(
                        LoxError::error(self.line, "Unexpected Char")
                    )
                }
            }

        }
        Ok(())
    }
    fn is_at_end(&self) -> bool{
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char{
        let res = *self.source.get(self.current).unwrap();
        self.current += 1;
        return res;
    }
    fn add_token(&mut self, token_type: TokenType){
        self.add_token_object(token_type, None);
    }
    fn add_token_object(&mut self, token_type: TokenType, literal: Option<Object>){
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push( Token::new(token_type, lexeme, literal, self.line) );
    }
    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end(){
            return false;
        } else if *self.source.get(self.current).unwrap() != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }

    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }
        return *self.source.get(self.current).unwrap()
    }
    fn peek_next(&self) -> char{
        if self.current +1 >= self.source.len() {
            return '\0'
        }
        return *self.source.get(self.current + 1).unwrap();
    }
    fn is_string(&mut self) -> Result<(), LoxError>{
        while self.peek() != '"' && !self.is_at_end(){
            if self.peek() == '\n' {
                self.line +=1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err( LoxError::error(self.line, "Unterminated String") );
        }
        self.advance();

        let value : String = self.source[(self.start+1)..(self.current-1)].iter().collect();
        //println!("{}", value);
        self.add_token_object(TokenType::String, Some(Object::Str(value)));

        return Ok(())
    }
    fn is_number(&mut self)-> Result<(), LoxError>{
        while self.peek().is_digit(10){
            self.advance();
        }
         if self.peek_next().is_digit(10) && self.peek() == '.'{
             self.advance();
             while self.peek().is_digit(10){
                 self.advance();
             }
         }
        let value: String = self.source[self.start..self.current].iter().collect();
        self.add_token_object(TokenType::Number, Some(Object::Num( value.parse::<f64>().unwrap() )));

        Ok(())
    }

    fn identifier(&mut self){
        while self.peek().is_alphanumeric() || self.peek() == '_'{
            self.advance();
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        let tt = self.keywords.get(&value);

        match tt {
            Some(tt) =>{
                self.add_token(*tt)
            }
            None => {
                self.add_token(TokenType::Identifier)
            }
        }

    }

}