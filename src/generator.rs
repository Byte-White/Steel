use crate::lexer::{Token, TokenType};
use std::fs::File;
use std::io::Write;

pub struct Generator{
    pub code:String,
}

impl Generator{
    pub fn new() -> Generator
    {
        Generator{
            code: String::new(),
        }
    }
    pub fn compile_line(&mut self, keyword:Token , args:Token)
    {
        match keyword.tokentype {
            TokenType::Exit => {
                //Handle Exit
            }
            TokenType::Window => {
                self.code.push_str(&format!("ImGui::Begin({});\n", args.text));
            }
            TokenType::Log => {
                match  keyword.text.as_str() {
                    "loginfo" => {
                        self.code.push_str(&format!("APPAZOID_INFO({});\n", args.text));
                    }
                    "logwarn" => {
                        self.code.push_str(&format!("APPAZOID_WARNING({});\n", args.text));
                    }
                    "logerror" => {
                        self.code.push_str(&format!("APPAZOID_ERROR({});\n", args.text));

                    }
                    "logcritical" => {
                        self.code.push_str(&format!("APPAZOID_CRITICAL({});\n", args.text));
                    }

                    _ => {
                        eprintln!("ERROR: invalid log");
                    }
                }
            }
            TokenType::Semicolon => {
                // Handle Semicolon
            }
            TokenType::String(_args) => {
                // Handle String
            }
            TokenType::Number(_args) => {
                // Handle Number
            }
            TokenType::Arguments(_args) => {
                // Handle Arguments
            }
            TokenType::Button => {
                self.code.push_str(&format!("Button({});\n", args.text));
            }
            TokenType::ColorPicker => {
                // Handle ColorPicker
            }
            TokenType::IntInput => {
                // Handle IntInput
            }
            TokenType::DoubleInput => {
                // Handle DoubleInput
            }
            TokenType::FloatInput => {
                // Handle FloatInput
            }
            _ => {
                // Handle all other cases (wildcard)
            }
        }
    }
    pub fn save_file_as(&mut self, filename:&str) -> Result<(), std::io::Error>
    {
        self.code += "ImGui::End();\n";
        let mut file = File::create(filename)?;

        file.write_all(self.code.as_bytes())?;

        Ok(())
    }
}