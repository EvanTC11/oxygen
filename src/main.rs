extern crate strum;
extern crate strum_macros;

use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;
use anyhow::{bail, Result};
use either::{ Left, Right};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use oxygen::parser::Parser;
use oxygen::{Statement};
use oxygen::token::{Token, TokenType};

fn main() -> Result<()> {
    let mut p = Parser {
        tokens: get_tokens("main.oxy")?
    };

    let mut stmts = p.parse()?;
    let mut f = File::create("out.c")?;
    f.write_all(statements_to_c(&mut stmts)?.as_bytes())?;

    Ok(())
}

fn get_tokens(path: &str) -> Result<Vec<Token>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut tokens = Vec::new();
    
    let mut buf = String::new();

    let mut chars = contents.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_alphabetic() {
            buf.push(c);

            while chars.peek().is_some_and(|next| next.is_alphabetic()) {
                buf.push(chars.next().unwrap());
            }

            match TokenType::from_str(&buf).unwrap_or(TokenType::NONE) {
                TokenType::RETURN => {
                    tokens.push(Token::new(TokenType::RETURN, String::new()));
                }
                TokenType::FUNCTION => {
                    tokens.push(Token::new(TokenType::FUNCTION, String::new()));
                }
                TokenType::INT => {
                    tokens.push(Token::new(TokenType::INT, String::new()));
                }
                TokenType::NONE => {
                    tokens.push(Token::new(TokenType::IDENTIFIER, buf.clone()));
                    buf.clear();
                }

                _ => buf.clear()
            }

            buf.clear();
        }

        else if c.is_numeric() {
            buf.push(c);
            while chars.peek().is_some_and(|next| next.is_numeric()) {
                buf.push(chars.next().unwrap());
            }

            tokens.push(Token::new(TokenType::INT_LIT, buf.clone()));
            buf.clear();
        }
        
        // else if TokenType::from_str(c.to_string().as_str())? == TokenType::MINUS {
        //     // either minus or operation TODO: impliment arithmetic
        //     buf.push(c);
        //     if chars.peek().unwrap().is_numeric() {
        //         buf.push(chars.next().unwrap());
        //         while chars.peek().is_some_and(|next| next.is_numeric()) {
        //             buf.push(chars.next().unwrap());
        //         }
        //         
        //         tokens.push(Token::new(TokenType::MINUS, String::new()));
        //         tokens.push(Token::new(TokenType::IDENTIFIER, buf.clone()));
        //     }
        //     
        //     buf.clear();
        // }

        else if c.is_whitespace() {
            continue;
        }

        else {
            for tok in TokenType::iter() {
                let try_tok = TokenType::from_str(c.to_string().as_str());
                if try_tok.is_ok() && try_tok? == tok {
                    tokens.push(Token::new(tok, String::new()));
                    break;
                }
            }
        }
    }

    Ok(tokens)
}

fn statements_to_c(stmts: &mut Vec<Statement>) -> Result<String> {
    let mut out = String::new();

    let mut stmts = stmts.iter_mut().peekable();
    while let Some(stmt) = stmts.next() {
        match stmt.expr {
            Left(ref expr) => {
                match expr.val {
                    Left(ref t) => {
                        match t.tok_type {
                            TokenType::RETURN => {
                                let ret_val = match &stmts.next().unwrap().expr {
                                    Left(expr) => match &expr.val {
                                        Left(token) => &token.val,
                                        Right(_) => bail!("Expected a literal return value"),
                                    },
                                    Right(_) => bail!("Expected a return expression"),
                                };

                                out.push_str(&format!("return {};", ret_val));
                            }

                            _ => todo!()
                        }
                    }

                    Right(ref bin_expr) => {
                        todo!()
                    }
                }
            }

            Right(ref mut func) => {
                // TODO: implement params
                let body = statements_to_c(&mut func.body)?;
                out += &format!("{} {}() {{ {} }}", func.ret_type.as_str(), func.name, body);
            }
        }
    }

    Ok(out)
}