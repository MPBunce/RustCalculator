use crate::error::*;
use crate::scanner::*;
use crate::ast_printer::*;
use crate::parser::*;
use std::io::{ self, BufRead, Write, stdout};
use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr};
use crate::interpreter::Interpreter;
use crate::token::Token;
use crate::token_type::TokenType;


pub struct Lox {
    interpreter: Interpreter
}
impl Lox {

    pub fn new() -> Lox {
        Lox { interpreter: Interpreter {} }
    }
    pub fn run_file(&mut self, filename: &str)-> io::Result<()> {

        let buf = std::fs::read_to_string(filename)?;
        match self.run(buf){
            Ok(_) => {},
            Err(_) => {
            //m.report( "".to_string());
            std::process::exit(65);
            }
        }
        Ok(())
    }

    pub fn run_prompt(&mut self){
        let stdin = io::stdin();
        print!("> ");
        let _ = stdout().flush();
        for line in stdin.lock().lines(){
        if let Ok(line) = line {
            if line.is_empty(){
                break;
            }
            match self.run(line) {
                Ok(_) => {}
                Err(_) => {
                //Ignore
                }
            }

        }else {
            break
        }
        print!("> ");
        let _ = stdout().flush();
        }
    }
    pub fn run(&mut self, source: String)-> Result<(), LoxError>{
        //println!("Running...");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        // println!("Done Scanning...");
        // for token in tokens {
        //     println!("{:?}", token);
        // }

        let t: Vec<Token> = tokens.iter().cloned().collect();
        let mut parser = Parser::new(t);
        match parser.parse() {
            Ok(expr) => {
                // If parsing is successful, do something with the expr
                self.interpreter.interpret(&expr);
            }
            Err(_) => {

            }
        }
        Ok(())
    }
}