
use std::process;
use crate::lexer::{Token, TokenType};
use crate::generator::Generator;

pub struct Parser {
    pub tokenlist: Vec<Token>,
    pub index: usize,
    pub generator: Generator,
    debug_print: bool,

}

impl Parser {
    pub fn new(tokens: Vec<Token>,debugp:bool, gen:Generator) -> Parser {
        Parser {
            tokenlist: tokens,
            index: 0,
            debug_print:debugp,
            generator: gen,
        }
    }

    pub fn parse(&mut self) {
        while self.index < self.tokenlist.len() {
            self.parse_statement();
        }
    }

    fn parse_statement(&mut self) {
        let current_token = self.peek();
        match  current_token.tokentype.clone(){
            TokenType::Exit => {
                self.consume(); // Consume 'exit'
                // Implement logic to handle 'exit' statement
                if self.debug_print{
                println!("Parsed 'exit' statement");
                }
            }
            TokenType::Window => {
                self.consume(); // Consume 'window'
                if self.debug_print{
                print!("Parsed 'window' statement ");
                }
                // Implement logic to handle 'return' statement
                if let TokenType::Arguments(arg_value) = &self.peek().tokentype {
                    // Use arg_value, which is a reference to the associated String
                    if self.debug_print{
                    println!("with Arguments : {}", arg_value);
                    }
                    // generates code.
                    self.generator.compile_line( self.tokenlist[self.index-1].clone(),self.tokenlist[self.index].clone());

                } else {
                    // Handle the case when TokenType is not Arguments
                    eprintln!("error! : no arguments for 'window' statement");
                    process::exit(1);
                }
                self.consume();
            }

            TokenType::EndWindow => {
                self.consume(); // Consume 'endwindow'
                if self.debug_print{
                print!("Parsed 'endwindow' statement ");
                }
                    self.generator.compile_line( self.tokenlist[self.index-1].clone(),self.tokenlist[self.index].clone());                
            }

            
            TokenType::Layer => {
                self.consume(); // Consume 'window'
                if self.debug_print{
                print!("Parsed 'layer' statement ");
                }
                // Implement logic to handle 'return' statement
                if let TokenType::Arguments(arg_value) = &self.peek().tokentype {
                    // Use arg_value, which is a reference to the associated String
                    if self.debug_print{
                    println!("with Arguments : {}", arg_value);
                    }
                    // generates code.
                    self.generator.compile_line( self.tokenlist[self.index-1].clone(),self.tokenlist[self.index].clone());

                } else {
                    // Handle the case when TokenType is not Arguments
                    eprintln!("error! : no arguments for 'layer' statement");
                    process::exit(1);
                }
                self.consume();
            }

            TokenType::EndLayer => {
                self.consume(); // Consume 'endlayer'
                if self.debug_print{
                print!("Parsed 'endlayer' statement ");
                }
                    self.generator.compile_line( self.tokenlist[self.index-1].clone(),self.tokenlist[self.index].clone());                
            }


            TokenType::Log => {
                self.consume(); // Consume 'loginfo', 'logwarn', or 'logcritical'
                // Implement logic to handle 'log' statement
                if self.debug_print{
                print!("Parsed 'log' statement ");
                }
                if let TokenType::Arguments(arg_value) = &self.peek().tokentype {
                    // Use arg_value, which is a reference to the associated String
                    if self.debug_print{
                    println!("with Arguments : {}", arg_value);
                    }
                    self.generator.compile_line( self.tokenlist[self.index-1].clone(),self.tokenlist[self.index].clone());
                } else {
                    // Handle the case when TokenType is not Arguments
                    eprintln!("error! : no arguments for log");
                    process::exit(1);
                }
                self.consume();
            }
            TokenType::Semicolon =>
            {
                self.consume();
                if self.debug_print{
                println!("Semicolon");
                }
            }
            TokenType::String(args) => {
                self.consume();
                if self.debug_print{
                println!("String '{}' skipped",&args);
                }
            }
            TokenType::Number(args) => {
                self.consume();
                if self.debug_print{
                println!("Number {} skipped",&args);
                }
            }
            TokenType::Arguments(args) => {
                self.consume();
                if self.debug_print{
                println!("Arguments '{}' skipped",&args);
                }
            }
            TokenType::Button =>{
                self.consume();
                
                if self.debug_print{
                println!("Parsed 'button' statement");
                }
                if let TokenType::Arguments(arg_value) = &self.peek().tokentype {
                    // Use arg_value, which is a reference to the associated String
                    if self.debug_print{
                    println!("with Arguments : {}", arg_value);
                    }
                    // generates code.
                    self.generator.compile_line( self.tokenlist[self.index-1].clone(),self.tokenlist[self.index].clone());

                } else {
                    // Handle the case when TokenType is not Arguments
                    eprintln!("error! : no arguments for 'button' statement");
                    process::exit(1);
                }
            }
            TokenType::ColorPicker =>{
                self.consume();
                if self.debug_print{
                println!("Parsed 'colorpicker' statement");
                }
            }
            TokenType::IntInput =>{
                self.consume();
                if self.debug_print{
                println!("Parsed 'intinput' statement");
                }
            }
            TokenType::DoubleInput =>{
                self.consume();
                if self.debug_print{
                println!("Parsed 'doubleinput' statement");
                }
            }
            TokenType::FloatInput =>{
                self.consume();
                if self.debug_print{
                println!("Parsed 'floatinput' statement");
                }
            }
            _ => {
                eprintln!("ERROR! : Invalid Token!");
                process::exit(1);
            }
        }
    }

    fn peek(&self) -> &Token {
        &self.tokenlist[self.index]
    }

    fn consume(&mut self) {
        self.index += 1;
    }
    pub fn compile(&mut self)
    {
        let _ = self.generator.save_file_as("az_compiled.h");
    }
}
