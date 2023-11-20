use std::process;
#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenType {
    None,
    Number(i32),
    String(String),
    Arguments(String),
    Exit,
    Log,
    Semicolon,
    Window,
    Button,
    Text,
    ColorPicker,
    TextInput,
    IntInput,
    DoubleInput,
    FloatInput,
}

#[derive(Clone)]
pub struct Token {
    pub tokentype: TokenType,
    pub text: String,
}

pub struct Lexer {
    pub tokenlist: Vec<Token>,
    pub source: String,
    pub index: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            tokenlist: Vec::new(),
            source: String::from(src),
            index: 0,
        }
    }
    
    pub fn lex(&mut self) {
        while self.index < self.source.len() {
            let current_char = self.peek();

            if current_char.is_whitespace() {
                // Skip whitespace
                self.index += 1;
            } else if current_char.is_digit(10) {
                // Lex a number
                let start = self.index;
                while self.peek().is_digit(10) { //checks the next symbol
                    self.index += 1;
                }
                let number_text = self.source[start..self.index].to_string(); 
                let number_value = match number_text.parse::<i32>() {
                    Ok(value) => value,
                    Err(err) => {
                        println!("ERROR: {0}",err);
                        0
                    }
                }; // Parse the number
                self.tokenlist.push(Token {
                    tokentype: TokenType::Number(number_value),
                    text: number_text,
                });
            } else if current_char == '"' {
                // Lex a string
                self.index += 1;// Skip the opening quote
                let start = self.index;
                while self.peek() != '"' {
                    self.index += 1;
                }
                let string_text = self.source[start..self.index].to_string();
                self.index += 1; // Skip the closing quote
                self.tokenlist.push(Token {
                    tokentype: TokenType::String(string_text.clone()),
                    text: string_text,
                });
            }
            else if current_char == '('
            {
                self.index += 1;
                let start = self.index;
                while self.peek() != ')'
                {
                    self.index += 1;
                }
                let string_text = self.source[start..self.index].to_string();
                self.index += 1; // skip closing ')';
                self.tokenlist.push(Token{
                    tokentype: TokenType::Arguments(string_text.clone()),
                    text: string_text,
                });
            } 
            else {
                // Lex other tokens
                let start = self.index;
                while !self.peek().is_whitespace() && self.peek() != '\0' && self.peek()!='(' {
                    self.index += 1;
                }
                let word_text = self.source[start..self.index].to_string();

                let token_type = match word_text.as_str() {
                    "exit" => TokenType::Exit,
                    "loginfo" => TokenType::Log,
                    "logwarn" => TokenType::Log,
                    "logerror" => TokenType::Log,
                    "logcritical" => TokenType::Log,
                    ";" => TokenType::Semicolon,
                    "window" => TokenType::Window,
                    "button" => TokenType::Button,
                    "text" => TokenType::Text,
                    "colorpicker" => TokenType::ColorPicker,
                    "intinput" => TokenType::IntInput,
                    "doubleinput" => TokenType::DoubleInput,
                    "floatinput" => TokenType::FloatInput,
                    "textinput" => TokenType::TextInput,
                    _ => TokenType::None,
                };
                if token_type == TokenType::None {
                    eprintln!("Error! : (index:{0}-{1}) '{2}' is invalid.",start,self.index,word_text);
                    process::exit(1);
                }
                self.tokenlist.push(Token {
                    tokentype: token_type,
                    text: word_text,
                });

                //self.index += 1; // Move past the whitespace
            }
        }
    }

    pub fn peek(&mut self) -> char {
        self.source.chars().nth(self.index).unwrap_or('\0')
    }
}
