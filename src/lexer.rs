use std::{
    collections::HashMap, env::set_var, error::Error, fmt::Display, fs::File, io::BufReader,
    iter::Peekable, str::Chars,
};

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    INT,
    INST_PUSH,
    INST_POP,
    INST_CMPE,
    INST_CMPNE,
    INST_DUP,
    INST_ADD,
    INST_SWAP,
    INST_SUB,
    INST_MUL,
    INST_DIV,
    INST_PRINT,
    INST_ZJMP,
    INST_NZJMP,
    INST_JP,
    INST_CMPG,
    INST_CMPL,
    INST_MOD,
    INST_CMPGE,
    INST_CMPLE,
    INST_NOP,
    INST_HALT,
    INST_INDUP,
    INST_ISWAP,
}
#[derive(Debug)]
pub struct Token {
    type_: TokenType,
    value: Option<i32>,
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.type_ {
            TokenType::INT => write!(f, "INT ({})", self.value.unwrap()),
            TokenType::INST_ADD => write!(f, "ADD"),
            TokenType::INST_PUSH => write!(f, "PUSH"),
            TokenType::INST_POP => write!(f, "POP"),
            _ => panic!("uknown token"),
        }
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    data: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
}
impl<'a> Lexer<'a> {
    pub fn read_source(data: &'a str) -> Self {
        let mut map = HashMap::new();
        map.insert("PUSH".to_owned(), TokenType::INST_PUSH);
        map.insert("POP".to_owned(), TokenType::INST_POP);
        map.insert("CMPE".to_owned(), TokenType::INST_CMPE);
        map.insert("DUP".to_owned(), TokenType::INST_DUP);
        map.insert("ADD".to_owned(), TokenType::INST_ADD);
        Self {
            data: data.chars().peekable(),
            tokens: vec![],
            keywords: map,
        }
    }
    pub fn push_token(&mut self, type_: TokenType, value: Option<i32>) {
        let token = Token { type_, value };
        self.tokens.push(token);
    }
    pub fn lexe(&mut self) -> Result<(), String> {
        while let Some(a) = self.data.next() {
            match a {
                x if x.is_ascii_whitespace() || x == '\n' => {}
                x if x.is_ascii_digit() && !x.is_ascii_whitespace() => {
                    let mut digit = String::new();
                    digit.push(x);
                    while let Some(a) = self.data.peek()
                        && a.is_ascii_digit()
                    {
                        digit.push(self.data.next().unwrap());
                    }
                    println!("=>{}", digit);
                    let data: i32 = digit.parse().unwrap();
                    self.push_token(TokenType::INT, Some(data));
                }
                x if x.is_ascii_alphabetic() && !x.is_ascii_whitespace() => {
                    let mut key = String::new();
                    key.push(x);
                    while let Some(a) = self.data.peek()
                        && a.is_ascii_alphabetic()
                    {
                        let d = self.data.next().unwrap();
                        key.push(d);
                    }
                    println!("=>{}", key);
                    let token = self.keywords.get(&key);
                    if let Some(a) = token {
                        self.push_token(*a, None);
                    } else {
                        return Err(format!("uknown keyword {key}"));
                    }
                }
                _ => return Err(format!("uknown symbol {a}")),
            }
        }
        Ok(())
    }
}
