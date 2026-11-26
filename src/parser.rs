use anyhow::{anyhow, bail, Result};
use either::{Left, Right};
use crate::{get_precedence, BinExpr, Expr, TokenOperation, Statement, StatementType, Function};
use crate::token::{Consume, DataType, Next, Token, TokenType};

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    fn term_parse(&mut self) -> Result<Token> {
        let tok = self.tokens.consume();
        match tok.tok_type {
            TokenType::INT_LIT | TokenType::IDENTIFIER => {
                Ok(tok)
            },

            TokenType::OPEN_BRACKET => {
                let expr = self.expr_parse(0);
                if expr.is_err() {
                    bail!("Expression expected")
                }

                Ok(expr?.val.unwrap_left().clone())
            }

            _ => {
                bail!("Invalid token {:?}", tok);
            }
        }
    }

    fn expr_parse(&mut self, min_prec: u8) -> Result<Expr> {
        let lhs = self.term_parse();
        if lhs.is_err() {
            bail!("Expression expected")
        }

        let lhs = lhs?;
        let mut expr_lhs = Expr {
            val: Right(BinExpr::empty())
        };

        expr_lhs.val = Left(lhs);

        loop {
            let curr = self.tokens.next().ok_or(anyhow!("Unexpected EOF"))?;
            let mut prec = get_precedence(curr.tok_type)?;
            if prec < min_prec {
                break;
            }

            self.tokens.consume();
            prec += 1;
            let expr_rhs = self.expr_parse(prec)?;
            let expr = match self.tokens.consume().tok_type {
                TokenType::PLUS => {
                    BinExpr {
                        lhs: Box::new(Expr {
                            val: expr_lhs.val
                        }),

                        rhs: Box::new(Expr {
                            val: expr_rhs.val
                        }),

                        operation: TokenOperation::Add
                    }
                }

                TokenType::MINUS => {
                    BinExpr {
                        lhs: Box::new(Expr {
                            val: expr_lhs.val
                        }),

                        rhs: Box::new(Expr {
                            val: expr_rhs.val
                        }),

                        operation: TokenOperation::Subtract
                    }
                }

                TokenType::FORWARD_SLASH => {
                    BinExpr {
                        lhs: Box::new(Expr {
                            val: expr_lhs.val
                        }),

                        rhs: Box::new(Expr {
                            val: expr_rhs.val
                        }),

                        operation: TokenOperation::Divide
                    }
                }

                TokenType::ASTERIX => {
                    BinExpr {
                        lhs: Box::new(Expr {
                            val: expr_lhs.val
                        }),

                        rhs: Box::new(Expr {
                            val: expr_rhs.val
                        }),

                        operation: TokenOperation::Multiply
                    }
                }

                _ => bail!("Invalid expression")
            };

            expr_lhs.val = Right(expr);
        }

        Ok(expr_lhs)
    }

    fn statement_parse(&mut self) -> Result<Statement> {
        let tok = self.tokens.consume();
        match tok.tok_type {
            TokenType::FUNCTION => {
                let ret_type = self.tokens.consume();
                if ret_type.tok_type != TokenType::INT { // TODO: Add more return types
                    bail!("Invalid return type")
                }

                let ret_type = match ret_type.tok_type {
                    TokenType::INT => DataType::INT,
                    _ => bail!("Invalid return type"),
                };

                let name = self.tokens.consume();
                if name.tok_type != TokenType::IDENTIFIER {
                    bail!("Invalid identifier")
                }

                let open_bracket = self.tokens.consume();
                let closed_bracket = self.tokens.consume();

                if open_bracket.tok_type != TokenType::OPEN_BRACKET || closed_bracket.tok_type != TokenType::CLOSE_BRACKET {
                    bail!("Invalid argument(s)")
                }

                if self.tokens.consume().tok_type != TokenType::OPEN_CURLY_BRACKET {
                    bail!("Invalid curly bracket placement")
                }

                let tok = self.tokens.consume();
                match tok.tok_type {
                    TokenType::RETURN => {
                        let ret_val = self.tokens.consume();
                        if ret_val.tok_type != TokenType::INT_LIT {
                            bail!("Unexpected token: expected int literal")
                        }

                        let ret_val = ret_val.val;

                        if self.tokens.consume().tok_type != TokenType::SEMI_COLON {
                            bail!("Expected semicolon")
                        }

                        if self.tokens.consume().tok_type != TokenType::CLOSE_CURLY_BRACKET { // NOTE: Once returning from func parser expects next token to be curly bracket
                            bail!("Expected closing curly bracket")
                        }

                        Ok(Statement {
                            expr: Right(Box::new(Function {
                                name: name.val,
                                ret_type,
                                params: None,
                                body: vec![Statement {
                                    expr: Left(Box::new(Expr { val: Left(tok) })),
                                    identifier: None,
                                    stmt_type: StatementType::RETURN,
                                },
                                           Statement {
                                               expr: Left(Box::new(Expr { val: Left(Token::new(TokenType::INT_LIT, ret_val)) })),
                                               identifier: None,
                                               stmt_type: StatementType::INT_LIT
                                           }
                                ]
                            })),

                            identifier: None,
                            stmt_type: StatementType::FUNCTION,
                        })
                    }
                    _ => bail!("Invalid token: {:?}", tok)
                }
            }
            _ => {
                bail!("Unexpected token: {:?}", tok)
            }
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        let mut stmts: Vec<Statement> = Vec::new();
        while self.tokens.next().is_some() {
            stmts.push( self.statement_parse().map_err(|e| anyhow!("{}", e))?);
        }

         Ok(stmts)
    }
}
