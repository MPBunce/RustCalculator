use crate::error::LoxError;
use crate::token::*;
use crate::expr::*;
use crate::token_type::*;
use crate::object::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Parser {
           Parser{
               tokens: input,
               current: 0
           }
    }

    pub fn parse(&mut self)-> Result<Expr, LoxError>{
        match self.expression() {
            Ok(r) => {
                return Ok(r)
            }
            Err(e) => {
                return Err(LoxError::error( 1 , "Error in parser primary function"))
            }
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr: Expr = self.comparison().unwrap();
        while self.is_match( &[TokenType::BangEqual, TokenType::EqualEqual] ){
            let operator = self.previous();
            let right = self.comparison().unwrap();
            expr = Expr::Binary( BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            })
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term().unwrap();
        while self.is_match(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual] ){
            let operator = self.previous();
            let right = self.factor().unwrap();
            expr = Expr::Binary( BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            })
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor().unwrap();
        while self.is_match(&[TokenType::Minus, TokenType::Plus ] ){
            let operator = self.previous();
            let right = self.factor().unwrap();
            expr = Expr::Binary( BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            })
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary().unwrap();
        while self.is_match(&[TokenType::Star, TokenType::Slash ] ){
            let operator = self.previous();
            let right = self.unary().unwrap();
            expr = Expr::Binary( BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            })
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match( &[TokenType::Bang, TokenType::Minus] ){
            let operator = self.previous();
            let right = self.unary().unwrap();
            return Ok(
                Expr::Unary( UnaryExpr {
                    operator,
                    right: Box::new(right)
                })
            )
        }
        let a = self.primary().unwrap();
        Ok(a)
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match( &[TokenType::False] ) {
            return Ok(
                Expr::Literal( LiteralExpr {
                    value: Some(Object::Bool(false))
                })
            )
        }
        if self.is_match(&[TokenType::True]) {
            return Ok(
                Expr::Literal( LiteralExpr {
                    value: Some(Object::Bool(true))
                })
            )
        }
        if self.is_match(&[TokenType::Nil]) {
            return Ok(
                Expr::Literal( LiteralExpr {
                    value: Some(Object::Nil)
                })
            )
        }
        if self.is_match(&[TokenType::String]) {
            return Ok(
                Expr::Literal( LiteralExpr {
                    value: self.previous().literal
                })
            )
        }
        if self.is_match(&[TokenType::Number]) {
            return Ok(
                Expr::Literal( LiteralExpr {
                    value: self.previous().literal
                })
            )
        }
        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression().unwrap();
            let _t = self.consume(TokenType::RightParen, "Expected ')' after expression".to_string() ).unwrap();
            return Ok(
                Expr::Grouping( GroupingExpr{
                    expression: Box::new(expr)
                })
            )
        }


        //Will never reach this
        return Err( LoxError::error( 1 , "Error in parser primary function" ) );
    }

    fn consume(&mut self, token_types: TokenType, s: String) -> Result<Token, LoxError> {
        if self.check(token_types){
            return Ok( self.advance() )
        }else{
            let t = self.peek();
            return Err( LoxError::parse_error(&t, &s) );
        }
    }

    fn synchronize(&mut self){
        let _t = self.advance();
        while !self.is_at_end(){

            if self.previous().token_type == TokenType::Semicolon {
                return
            }
             match self.peek().token_type {
                 TokenType::Class => {}
                 TokenType::Fun => {}
                 TokenType::Var => {}
                 TokenType::For => {}
                 TokenType::If => {}
                 TokenType::While => {}
                 TokenType::Print => {}
                 TokenType::Return => {
                     return;
                 }
                 _ => {
                     return;
                 }
             }

            let _ = self.advance();
        }
    }

    fn is_match(&mut self, token_types: &[TokenType] ) ->  bool  {
        //println!("match?");
        for t in token_types {
            //println!("{:?}", &t);
            if self.check(*t){
                self.advance();
                return true
            }
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            return self.peek().token_type == token_type
        }

    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;

        }
        //println!("advance");
        return self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).cloned().unwrap()
    }

    fn previous(&self) -> Token {
        //println!("Len: {} Current {}", self.tokens.len(), self.current);
        self.tokens.get(self.current-1).cloned().unwrap()
    }


}