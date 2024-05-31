mod error;
use error::*;
mod scanner;
use scanner::*;
mod token;
mod token_type;
use token_type::*;
mod ast_printer;
use ast_printer::*;
mod expr;
mod parser;
mod interpreter;
mod object;
mod lox;
use lox::*;
use parser::*;

use expr::*;

use std::env;
use std::io::{ self, BufRead, Write, stdout};
use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::token::Token;
use crate::token_type::TokenType;

fn main() {
    let mut lox = Lox::new();

    lox.run_prompt();
    
}

//For testing AST Printer
// fn main(){
//     let expression = Expr::Binary(
//         BinaryExpr {
//             left: Box::new( Expr::Unary(
//                 UnaryExpr {
//                     operator: Token {
//                         token_type: TokenType::Minus,
//                         lexeme: "-".to_string(),
//                         literal: None,
//                         line: 1
//                     },
//                     right: Box::new(Expr::Literal( LiteralExpr { value: Some(token::Object::Num( f64::from(123) )) }))
//
//                 }
//             )),
//             operator: Token {
//                 token_type: TokenType::Star,
//                 lexeme: "*".to_string(),
//                 literal: None,
//                 line: 1
//             },
//             right: Box::new( Expr::Grouping(
//                 GroupingExpr {
//                     expression: Box::new(Expr::Literal( LiteralExpr {value: Some(token::Object::Num(45.67))} ))
//                 }
//             ))
//         }
//     );
//
//     let printer = AstPrinter {};
//     println!("{:?}", printer.print(&expression).unwrap() )
//
// }

//Moved to lox.rs
// fn run_file(filename: &str)-> io::Result<()> {
//
//     let buf = std::fs::read_to_string(filename)?;
//     match run(buf){
//         Ok(_) => {},
//         Err(_) => {
//             //m.report( "".to_string());
//             std::process::exit(65);
//         }
//     }
//     Ok(())
//
// }
//
// fn run_prompt(){
//     let stdin = io::stdin();
//     print!("> ");
//     let _ = stdout().flush();
//     for line in stdin.lock().lines(){
//         if let Ok(line) = line {
//             if line.is_empty(){
//                 break;
//             }
//             match run(line) {
//                 Ok(_) => {}
//                 Err(_) => {
//                     //Ignore
//                 }
//             }
//
//         }else {
//             break
//         }
//         print!("> ");
//         let _ = stdout().flush();
//     }
// }
//
// fn run(source: String)-> Result<(), LoxError>{
//     println!("Running...");
//     let mut scanner = Scanner::new(source);
//     let tokens = scanner.scan_tokens()?;
//     // println!("Done Scanning...");
//     // for token in tokens {
//     //     println!("{:?}", token);
//     // }
//
//     let t: Vec<Token> = tokens.iter().cloned().collect();
//     let mut parser = Parser::new(t);
//     let expr = parser.start_parse().unwrap();
//     let printer = AstPrinter {};
//     println!("{:?}", printer.print(&expr).unwrap() );
//
//     Ok(())
// }
//
