use std::str::FromStr;
use anyhow::{bail, Result, anyhow};
use strum_macros::{EnumIter};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DataType {
    INT,
}

impl FromStr for DataType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "int" => Ok(DataType::INT),

            _ => bail!("Unknown token type {}", s),
        }
    }
}

impl DataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DataType::INT => "int",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum TokenType {
    NONE,

    FUNCTION,
    RETURN,

    INT,
    INT_LIT,

    SEMI_COLON,

    OPEN_BRACKET,
    CLOSE_BRACKET,

    OPEN_CURLY_BRACKET,
    CLOSE_CURLY_BRACKET,

    IDENTIFIER,

    MINUS,
    PLUS,
    FORWARD_SLASH,
    ASTERIX
}
impl FromStr for TokenType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "fn" => Ok(TokenType::FUNCTION),
            "ret" => Ok(TokenType::RETURN),

            "int" => Ok(TokenType::INT),

            ";" => Ok(TokenType::SEMI_COLON),

            "(" => Ok(TokenType::OPEN_BRACKET),
            ")" => Ok(TokenType::CLOSE_BRACKET),

            "{" => Ok(TokenType::OPEN_CURLY_BRACKET),
            "}" => Ok(TokenType::CLOSE_CURLY_BRACKET),

            "-" => Ok(TokenType::MINUS),

            _ => Err(anyhow!("Unknown token type {}", s)),
        }
    }
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::FUNCTION => "_",
            TokenType::RETURN => "return",

            TokenType::INT => "int",

            TokenType::SEMI_COLON => ";",

            TokenType::OPEN_BRACKET => "(",
            TokenType::CLOSE_BRACKET => ")",

            TokenType::OPEN_CURLY_BRACKET => "{",
            TokenType::CLOSE_CURLY_BRACKET => "}",

            _ => "NONE",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub tok_type: TokenType,
    pub val: String,
}

impl Token {
    pub fn new(tok_type: TokenType, val: String) -> Self {
        Self { tok_type, val }
    }
}

pub trait Consume {
    type Item;
    fn consume(&mut self) -> Self::Item;
}

pub trait Next {
    type Item;
    fn next(&self) -> Option<Self::Item>;
}

impl<T> Consume for Vec<T> {
    type Item = T;
    fn consume(&mut self) -> T {
        self.remove(0)
    }
}

impl<T: Clone> Next for Vec<T> {
    type Item = T;
    fn next(&self) -> Option<T> {
        self.get(0).clone().cloned()
    }
}