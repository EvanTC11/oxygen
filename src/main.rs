extern crate strum;
#[macro_use] extern crate strum_macros;

use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use anyhow::{Result, anyhow};
use oxygen::token::{TokenType, DataType, Token, Consume};


fn main() -> Result<()> {
    let c = file_parse("main.oxy")?;
    println!("{:#?}", c);

    Ok(())
}

fn file_parse(in_path: &str) -> Result<String> {
    let mut file = File::open(in_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tokens_raw = contents.trim().split_whitespace().collect::<Vec<&str>>();
    let mut tokens: Vec<Token> = Vec::new();
    for token in &tokens_raw {
        tokens.push(Token {
            tok_type: token.parse()?,
            val: token.parse()?,
        })
    }

    let mut out = String::new();

    for (i, token) in tokens.clone().iter().enumerate() {
        match token.tok_type {
            TokenType::FUNCTION => {
                let mut fn_tokens: Vec<TokenType> = Vec::new();

                out += format!("{} {}", tokens[i + 1].val.parse::<DataType>().map_err(|e| anyhow!(e))?.as_str(), tokens[i + 2].val).as_str(); // TODO: use consume()

                tokens.consume();
                tokens.consume();
                tokens.consume();
                if tokens[i].tok_type != TokenType::OPEN_BRACKET {
                    panic!("Invalid token {:?}", tokens[i]);
                };

                fn_tokens.push(tokens[i].tok_type.clone());

                tokens.consume();
                if tokens[i].tok_type != TokenType::CLOSE_BRACKET {
                    panic!("Invalid token {:?}", tokens[i]);
                }

                fn_tokens.push(tokens[i].tok_type.clone());

                tokens.consume();
                if tokens[i].tok_type != TokenType::OPEN_CURLY_BRACKET {
                    panic!("Invalid token {:?}", tokens[i]);
                }

                fn_tokens.push(tokens[i].tok_type.clone());

                fn_tokens.iter().for_each(|t| out += t.as_str() );

                tokens.consume();
                match tokens[i].tok_type {
                    TokenType::RETURN => {
                        let mut ret_tokens: Vec<TokenType> = Vec::new();

                        let prev = tokens[i].clone();
                        tokens.consume();
                        let ret_val = tokens[i].val.parse::<i16>().map_err(|e| anyhow!(e))?;
                        out += format!("{} {}", prev.tok_type.as_str(), ret_val).as_str();

                        tokens.consume();
                        if tokens[i].tok_type != TokenType::SEMI_COLON {
                            panic!("Invalid token {:?}", tokens[i]);
                        }

                        ret_tokens.push(tokens[i].tok_type);

                        tokens.consume();
                        if tokens[i].tok_type != TokenType::CLOSE_CURLY_BRACKET {
                            panic!("Invalid token {:?}", tokens[i]);
                        }

                        ret_tokens.push(tokens[i].tok_type);
                        ret_tokens.iter().for_each(|t| out += t.as_str());
                    },

                    _ => {
                        todo!();
                    }
                }
            }

            _ => continue,
        }
    }

    Ok(out)
}