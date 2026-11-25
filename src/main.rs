extern crate strum;
#[macro_use] extern crate strum_macros;

use std::cmp::PartialEq;
use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use anyhow::{bail, Result, anyhow};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TokenType {
    NONE,

    FUNCTION,
    RETURN,

    INT,

    SEMI_COLON,

    OPEN_BRACKET,
    CLOSE_BRACKET,

    OPEN_CURLY_BRACKET,
    CLOSE_CURLY_BRACKET,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DataType {
    INT,
}

impl FromStr for TokenType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "fn" => Ok(TokenType::FUNCTION),
            "ret" => Ok(TokenType::RETURN),

            ";" => Ok(TokenType::SEMI_COLON),

            "(" => Ok(TokenType::OPEN_BRACKET),
            ")" => Ok(TokenType::CLOSE_BRACKET),

            "{" => Ok(TokenType::OPEN_CURLY_BRACKET),
            "}" => Ok(TokenType::CLOSE_CURLY_BRACKET),

            _ => Ok(TokenType::NONE),
        }
    }
}

impl FromStr for DataType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "int" => Ok(DataType::INT),

            _ => bail!("Unknown token type {}", s),
        }
    }
}

impl DataType {
    fn as_str(&self) -> &'static str {
        match self {
            DataType::INT => "int",
        }
    }
}

impl TokenType {
    fn as_str(&self) -> &'static str {
        match self {
            TokenType::FUNCTION => "_",
            TokenType::RETURN => "return",

            TokenType::SEMI_COLON => ";",

            TokenType::OPEN_BRACKET => "(",
            TokenType::CLOSE_BRACKET => ")",

            TokenType::OPEN_CURLY_BRACKET => "{",
            TokenType::CLOSE_CURLY_BRACKET => "}",

            _ => "NONE",
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut file = File::open("src/main.oxy")?;
    let mut out = File::create("src/out.c")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tokens_str = contents.trim().split_ascii_whitespace().collect::<Vec<&str>>();
    let tokens = tokens_str.iter().map(|t| t.parse::<TokenType>().unwrap()).collect::<Vec<TokenType>>();

    for (mut i, token) in tokens.iter().enumerate() {
        match token {
            TokenType::FUNCTION => {
                let mut func = String::new();
                let mut fn_tokens: Vec<TokenType> = Vec::new();

                func += format!("{} {}", tokens_str[i + 1].parse::<DataType>().map_err(|e| anyhow!(e))?.as_str(), tokens_str[i + 2]).as_str();

                i += 3;
                if tokens[i] != TokenType::OPEN_BRACKET {
                    panic!("Invalid token {:?}", tokens[i]);
                };

                fn_tokens.push(tokens[i]);

                i += 1;
                if tokens[i] != TokenType::CLOSE_BRACKET {
                    panic!("Invalid token {:?}", tokens[i]);
                }

                fn_tokens.push(tokens[i]);

                i += 1;
                if tokens[i] != TokenType::OPEN_CURLY_BRACKET {
                    panic!("Invalid token {:?}", tokens[i]);
                }

                fn_tokens.push(tokens[i]);

                fn_tokens.iter().for_each(|t| func += t.as_str() );
                func += "\n";

                i += 1;
                match tokens[i] {
                    TokenType::RETURN => {
                        let mut ret_tokens: Vec<TokenType> = Vec::new();

                        i += 1;
                        let ret_val = tokens_str[i].parse::<i16>();
                        if ret_val.is_err() {
                            panic!("Invalid data type {:?}", tokens_str[i]);
                        }

                        func += format!("{} {}", tokens[i - 1].as_str(), ret_val?).as_str();

                        i += 1;
                        if tokens[i] != TokenType::SEMI_COLON {
                            panic!("Invalid token {:?}", tokens[i]);
                        }

                        ret_tokens.push(tokens[i]);

                        i += 1;
                        if tokens[i] != TokenType::CLOSE_CURLY_BRACKET {
                            panic!("Invalid token {:?}", tokens[i]);
                        }

                        ret_tokens.push(tokens[i]);
                        ret_tokens.iter().for_each(|t| func += t.as_str());
                    },

                    _ => {
                        todo!();
                    }
                }

                out.write_all(func.as_bytes())?;
            }

            _ => continue,
        }
    }

    Ok(())
}
