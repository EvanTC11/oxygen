use std::str::FromStr;
use anyhow::bail;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
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
impl FromStr for TokenType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
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

impl TokenType {
    pub fn as_str(&self) -> &'static str {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub tok_type: TokenType,
    pub val: String,
}

impl Token {
    
}

pub trait Consume {
    fn consume(&mut self) -> Token;
}

impl Consume for Vec<Token> {
    fn consume(&mut self) -> Token {
        self.remove(0)
    }
}