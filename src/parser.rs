use std::{
    cell::{RefCell, RefMut}, collections::HashMap, panic, rc::Rc
};

use crate::lexer::{self, Lexer, Token, TokenType, LEXVALUES};

#[derive(Debug, Clone)]
pub enum Literal {
    INT(i32),
    STRING(Rc<String>),
}

#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum ParseValue {
    PUSH { token: Token, value: Literal },
    POP { token: Token },
    CMPE { token: Token },
    CMPNE { token: Token },
    DUP { token: Token },
    ADD { token: Token },
    SWAP { token: Token },
    SUB { token: Token },
    MUL { token: Token },
    DIV { token: Token },
    PRINT { token: Token },
    ZJMP { token: Token, value: Literal },
    NZJMP { token: Token, value: Literal },
    JP { token: Token, value: Literal },
    CMPG { token: Token },
    CMPL { token: Token },
    MOD { token: Token },
    CMPGE { token: Token },
    CMPLE { token: Token },
    NOP { token: Token },
    HALT { token: Token },
    INDUP { token: Token, value: Literal },
    ISWAP { token: Token, value: Literal },
    EOF,
}

pub struct Parser {
    tokens: Vec<Token>,
    counter: usize,
    tree: Vec<ParseValue>,
    pub labels:HashMap<String,i32>
}

impl Parser {
    pub fn new<'a>(lexer: &'a mut Lexer<'a>) -> Self {
        let tokens = lexer.lexe().unwrap().to_vec();
        
        Self {
            tokens,
            counter: 0,
            tree: vec![],
            labels: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> &[ParseValue] {
        while self.tokens.len() > self.counter {
            self.parse_tokens();
            self.counter += 1;
        }
        self.tree.push(ParseValue::EOF);
        &self.tree
    }

    fn parse_tokens(&mut self) {
        let token = self.tokens[self.counter].clone();
        match token.type_ {
            lexer::TokenType::LABEL_DECL=>{
                let tk = token.clone();
                if let Some(LEXVALUES::STRING(a)) = token.value{
                    self.labels.insert(a.clone().to_string(), (token.line) as i32);
                }else {
                    panic!("Expected label");
                }
                self.tree.push(ParseValue::NOP { token:tk });
            }

            lexer::TokenType::INST_PUSH => {
                let token_int = self.consume(TokenType::INT, "expected int ");
                 let Some(LEXVALUES::INT(a)) = token_int.value else{
                    panic!("expected integer");
                };

                self.tree.push(ParseValue::PUSH {
                    token,
                    value: Literal::INT(a),
                });
            }
            TokenType::INST_POP => {
                self.tree.push(ParseValue::POP { token });
            }
            TokenType::INST_CMPE => {
                self.tree.push(ParseValue::CMPE { token });
            }
            TokenType::INST_CMPNE => {
                self.tree.push(ParseValue::CMPNE { token });
            }
            TokenType::INST_DUP => {
                self.tree.push(ParseValue::DUP { token });
            }
            TokenType::INST_ADD => {
                self.tree.push(ParseValue::ADD { token });
            }
            TokenType::INST_SWAP => {
                self.tree.push(ParseValue::SWAP { token });
            }
            TokenType::INST_SUB => {
                self.tree.push(ParseValue::SUB { token });
            }
            TokenType::INST_MUL => {
                self.tree.push(ParseValue::MUL { token });
            }
            TokenType::INST_DIV => {
                self.tree.push(ParseValue::DIV { token });
            }
            TokenType::INST_PRINT => {
                self.tree.push(ParseValue::PRINT { token });
            }
            TokenType::INST_ZJMP => {
                let token_int = self.consume(TokenType::IDENTIFIER, "expected NAME");
                 let Some(LEXVALUES::STRING(a)) = token_int.value else{
                    panic!("expected integer");
                };
    
                self.tree.push(ParseValue::ZJMP {
                    token,
                    value: Literal::STRING(a.clone()),
                });
            }
            TokenType::INST_NZJMP => {
              let token_int = self.consume(TokenType::IDENTIFIER, "expected NAME");
                 let Some(LEXVALUES::STRING(a)) = token_int.value else{
                    panic!("expected integer");
                };
    
                self.tree.push(ParseValue::NZJMP {
                    token,
                    value: Literal::STRING(a.clone()),
                });

            }
            TokenType::INST_JP => {
              let token_int = self.consume(TokenType::IDENTIFIER, "expected NAME");
                 let Some(LEXVALUES::STRING(a)) = token_int.value else{
                    panic!("expected integer");
                };
    
                self.tree.push(ParseValue::JP {
                    token,
                    value: Literal::STRING(a.clone()),
                });
            }
            TokenType::INST_CMPG => {
                self.tree.push(ParseValue::CMPG { token });
            }
            TokenType::INST_CMPL => {
                self.tree.push(ParseValue::CMPL { token });
            }

            TokenType::INST_MOD => {
                self.tree.push(ParseValue::MOD { token });
            }
            TokenType::INST_CMPGE => {
                self.tree.push(ParseValue::CMPGE { token });
            }
            TokenType::INST_CMPLE => {
                self.tree.push(ParseValue::CMPLE { token });
            }
            TokenType::INST_NOP => {
                self.tree.push(ParseValue::NOP { token });
            }
            TokenType::INST_HALT => {
                self.tree.push(ParseValue::HALT { token });
            }
            TokenType::INST_INDUP => {
                let token_int = self.consume(TokenType::INT, "expected INT");
                let Some(LEXVALUES::INT(a)) = token_int.value else{
                    panic!("expected integer");
                };
                self.tree.push(ParseValue::INDUP{
                    token,
                    value: Literal::INT(a),
                });

            }
            TokenType::INST_ISWAP => {
                let token_int = self.consume(TokenType::INT, "expected INT");
                let Some(LEXVALUES::INT(a)) = token_int.value else{
                    panic!("expected integer");
                };
                self.tree.push(ParseValue::ISWAP {
                    token,
                    value: Literal::INT(a),
                });
            }
            a => panic!("uknown {:?}", token),
        }
    }
    pub fn consume(&mut self, expects: TokenType, error: &str) -> Token {
        if self.tokens[self.counter + 1].type_ == expects {
            self.counter += 1;
            self.tokens[self.counter].clone()
        } else {
            panic!("{}", error);
        }
    }
}
