use anyhow::anyhow;
use either::Either;
use crate::token::{DataType, Token, TokenType};

pub mod token;
pub mod parser;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenOperation {
    NONE,

    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct BinExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,

    pub operation: TokenOperation,
}
impl BinExpr {
    pub fn empty() -> Self {
        Self {
            lhs: Box::new(Expr {
                val: Either::Left(Token::new(TokenType::NONE, String::new()))
            }),
            rhs: Box::new(Expr {
                val: Either::Left(Token::new(TokenType::NONE, String::new()))
            }),
            operation: TokenOperation::NONE,
        }
    }
}

#[derive(Debug)]
pub struct Expr {
    pub val: Either<Token, BinExpr>,
}

pub fn get_precedence(tok: TokenType) -> anyhow::Result<u8> {
    match tok {
        TokenType::MINUS | TokenType::PLUS => {
            Ok(0)
        }

        TokenType::FORWARD_SLASH | TokenType::ASTERIX => {
            Ok(1)
        }

        _ => Err(anyhow!("Invalid token {:?}", tok))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StatementType {
    INT_LIT,
    IDENTIFIER,
    FUNCTION,
    ASSIGN,
    RETURN
}

#[derive(Debug)]
pub struct Statement {
    pub expr: Either<Box<Expr>, Box<Function>>,
    pub identifier: Option<Token>,
    pub stmt_type: StatementType,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub ret_type: DataType,
    pub params: Option<Vec<Token>>,
    pub body: Vec<Statement>,
}