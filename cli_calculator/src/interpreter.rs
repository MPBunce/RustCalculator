use crate::error::LoxError;
use crate::expr::*;
use crate::object::*;
use crate::token_type::TokenType;

pub struct Interpreter;

impl ExprVisitor<Object> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left).unwrap();
        let right =self.evaluate(&expr.right).unwrap();

        //println!("left {:?}, expr: {:?}, right: {:?}", &left, &expr.operator.token_type, &right);

        let res = match expr.operator.token_type {
            TokenType::Minus => left - right,
            TokenType::Plus => left + right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            TokenType::Greater => Object::Bool(left > right),
            TokenType::GreaterEqual => Object::Bool(left >= right),
            TokenType::Less => Object::Bool(left < right),
            TokenType::LessEqual => Object::Bool(left <= right),
            TokenType::BangEqual => Object::Bool( left != right ),
            TokenType::EqualEqual => Object::Bool( left == right ),
            _ => {
                todo!("Add case");
            }
        };

        //println!("res:  {:?}", &res);

        if res == Object::ArithmeticError {
            Err( LoxError::runtime_error(&expr.operator, "Bad Expression") )
        } else {
            Ok(res)
        }

    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        Ok( self.evaluate( &expr.expression ).unwrap() )
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        let a = expr.value.clone().unwrap();
        Ok(a)
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right).unwrap();
        match expr.operator.token_type {
            TokenType::Minus => {
                if let Object::Num(right) = right {
                    return Ok( Object::Num( - right) )
                }
                return Err( LoxError::error(expr.operator.line, "Error Binary Exp" ) )
            }
            TokenType::Bang => {
                if self.is_truthy(right) {
                    Ok(Object::Bool(false))
                } else {
                    Ok(Object::Bool(true))
                }
            }
            _ => {
                return Ok(Object::Nil)
            }
        }
    }


}

impl Interpreter {
    pub fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
            expr.accept(self)
    }

    pub fn is_truthy(&self, obj: Object) -> bool {
        !matches!(obj, Object::Nil | Object::Bool(false))
    }

    pub fn interpret(&self, expr: &Expr) -> bool {
        match self.evaluate(&expr){
            Ok(v) => {
                println!("{}", v);
                true
            },
            Err(e) => {
                e.report("");
                false
            }
        }
    }

}