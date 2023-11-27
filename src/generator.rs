use crate::lexer::{Token, TokenType};
use std::fs::File;
use std::io::Write;
use regex::Regex;
use std::process;

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

            TokenType::EndWindow => {
                self.code.push_str(&format!("ImGui::End();\n"));
            }

            TokenType::Layer => {
                self.code.push_str(&format!("class {} : public az::Layer\n{{\npublic:\n\nvoid OnUIRender() override\n{{\n", args.text.replace("\"","")));
            }

            TokenType::EndLayer => {
                self.code.push_str(&format!("}}\n}};\n"));
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

                if args.text.contains('{') == false
                {
                    self.code.push_str(&format!("ImGui::Button({});\n", args.text));
                    //println!("--------------args: {}",args.text);
                }
                else 
                {
                    let start = args.text.find('{').unwrap_or(0);
                    let mut numofbrackets = 0; // used to handle nested brackets
                    let mut index = start;
                    while index < args.text.len() && (args.text.chars().nth(index) != Some('}') || numofbrackets != 0) {
                        if args.text.chars().nth(index) == Some('{') {
                            numofbrackets += 1;
                        } else if args.text.chars().nth(index) == Some('}') {
                            numofbrackets -= 1;
                        }
                        index += 1;
                    }
                    let string_text = &args.text[start..index]; // code when button is clicked
                    println!("~-~-~-~-~+++{0}",args.text);
                    let re = Regex::new(r#""([^"]*)""#).unwrap();
                    if let Some(captures) = re.captures(&args.text) {
                        if let Some(buttonname) = captures.get(0) {
                            println!("~-~-~-~-~---{0}",buttonname.as_str());
                            self.code.push_str(&format!("if(ImGui::Button({0}))\n{1}\n", buttonname.as_str(),string_text));
                        }
                        else {
                            println!("ERROR! REGEX error, no button pressed event");
                            process::exit(1);
                        }
                    }
                }
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
        let mut file = File::create(filename)?;

        file.write_all(self.code.as_bytes())?;

        Ok(())
    }
}