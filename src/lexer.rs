use std::{collections::HashMap, fmt::Display, iter::Peekable, rc::Rc, str::Chars};


#[derive(Debug,Clone)]
pub enum LEXVALUES{
    INT(i32),
    STRING(Rc<String>),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    INT,
    IDENTIFIER,
    LABEL_DECL,
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

#[derive(Debug, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub value: Option<LEXVALUES>,
    pub line: usize,
}


#[derive(Debug)]
pub struct Lexer<'a> {
    data: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    line: usize,
}
impl<'a> Lexer<'a> {
    pub fn read_source(data: &'a str) -> Self {
        let mut map = HashMap::new();
        map.insert("PUSH".to_lowercase(), TokenType::INST_PUSH);
        map.insert("POP".to_lowercase(), TokenType::INST_POP);
        map.insert("CMPE".to_lowercase(), TokenType::INST_CMPE);
        map.insert("DUP".to_lowercase(), TokenType::INST_DUP);
        map.insert("ADD".to_lowercase(), TokenType::INST_ADD);
        map.insert("CMPNE".to_lowercase(), TokenType::INST_CMPNE);
        map.insert("SWAP".to_lowercase(), TokenType::INST_SWAP);
        map.insert("SUB".to_lowercase(), TokenType::INST_SUB);
        map.insert("MUL".to_lowercase(), TokenType::INST_MUL);
        map.insert("DIV".to_lowercase(), TokenType::INST_DIV);

        map.insert("PRINT".to_lowercase(), TokenType::INST_PRINT);
        map.insert("ZJUMP".to_lowercase(), TokenType::INST_ZJMP);
        map.insert("NZJUMP".to_lowercase(), TokenType::INST_NZJMP);
        map.insert("CMPG".to_lowercase(), TokenType::INST_CMPG);
        map.insert("CMPL".to_lowercase(), TokenType::INST_CMPL);
        map.insert("MOD".to_lowercase(), TokenType::INST_MOD);
        map.insert("CMPGE".to_lowercase(), TokenType::INST_CMPGE);
        map.insert("CMPLE".to_lowercase(), TokenType::INST_CMPLE);

        map.insert("JUMP".to_lowercase(), TokenType::INST_JP);
        map.insert("NOP".to_lowercase(), TokenType::INST_NOP);
        map.insert("HALT".to_lowercase(), TokenType::INST_HALT);
        map.insert("INDUP".to_lowercase(), TokenType::INST_INDUP);
        map.insert("ISWAP".to_lowercase(), TokenType::INST_ISWAP);
        
        Self {
            data: data.chars().peekable(),
            tokens: vec![],
            keywords: map,
            line: 0,
        }
    }
    pub fn push_token(&mut self, type_: TokenType, value: Option<LEXVALUES>, line: usize) {
        let token = Token { type_, value, line };
        self.tokens.push(token);
    }
    // till the mutable reffrence is alive the result is alive
    pub fn lexe(&'a mut self) -> Result<&'a [Token], String> {
        while let Some(a) = self.data.next() {
            if a == '\n'{
                self.line +=1;
            }
            match a {
                x if x.is_ascii_whitespace() => {}
                x if x.is_ascii_digit() && !x.is_ascii_whitespace() => {
                    let mut digit = String::new();
                    digit.push(x);
                    while let Some(a) = self.data.peek()
                        && a.is_ascii_digit()
                    {
                        digit.push(self.data.next().unwrap());
                    }
                    //println!("=>{}", digit);
                    let data: i32 = digit.parse().unwrap();
                    self.push_token(TokenType::INT, Some(LEXVALUES::INT(data)), self.line);

                }
                x if x.is_ascii_alphabetic() && !x.is_ascii_whitespace() => {
                    let mut key = String::new();
                    key.push(x);
                    while let Some(a) = self.data.peek() &&  Self::is_valid(*a) {
                        let d = self.data.next().unwrap();
                        key.push(d);
                    }

                    let token = self.keywords.get(&key.to_lowercase());
                    if let Some(a) = token {
                        self.push_token(*a, None, self.line);
                    } else if key.contains(":"){
                        let key = key.replace(":", "");
                        self.push_token(TokenType::LABEL_DECL, Some(LEXVALUES::STRING(Rc::new(key))), self.line);
                    }else {
                        self.push_token(TokenType::IDENTIFIER, Some(LEXVALUES::STRING(Rc::new(key))), self.line);
                    }
                }
                _ => return Err(format!("uknown symbol {a}")),
            }
        }
        Ok(self.tokens.as_slice())
    }


    fn is_valid(a:char)->bool{
       a == ':' ||  a.is_ascii_alphabetic()
    }
}
