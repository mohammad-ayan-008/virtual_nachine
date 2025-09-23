use std::{fs::File, io::Write};

use crate::{
    instructions::{self, Inst_Set, Pad},
    parser::{Literal, ParseValue},
};

pub struct CodeGen {
    ast: Vec<ParseValue>,
}

impl CodeGen {
    pub fn new(values: &[ParseValue]) -> Self {
        Self {
            ast: values.to_vec(),
        }
    }

    #[allow(non_snake_case, dead_code, unused)]
    pub fn generat_(self, file_name: &str) {
        let mut code = Vec::<instructions::Inst_Set>::new();
        for i in self.ast {
            match i {
                ParseValue::PUSH { token, value } => {
                    let Literal::INT(value) = value;

                    code.push(instructions::Inst_Set::INST_PUSH { value });
                }
                ParseValue::POP { token } => {
                    code.push(instructions::Inst_Set::INST_POP { _pad: Pad::Padding });
                }
                ParseValue::CMPE { token } => {
                    code.push(instructions::Inst_Set::INST_CMPE { _pad: Pad::Padding });
                }
                ParseValue::CMPNE { token } => {
                    code.push(instructions::Inst_Set::INST_CMPNE { _pad: Pad::Padding });
                }
                ParseValue::DUP { token: Token } => {
                    code.push(instructions::Inst_Set::INST_DUP { _pad: Pad::Padding });
                }
                ParseValue::ADD { token } => {
                    code.push(instructions::Inst_Set::INST_ADD { _pad: Pad::Padding });
                }
                ParseValue::SWAP { token } => {
                    code.push(instructions::Inst_Set::INST_SWAP { _pad: Pad::Padding });
                }
                ParseValue::SUB { token } => {
                    code.push(instructions::Inst_Set::INST_SUB { _pad: Pad::Padding });
                }
                ParseValue::MUL { token } => {
                    code.push(instructions::Inst_Set::INST_MUL { _pad: Pad::Padding });
                }
                ParseValue::DIV { token: Token } => {
                    code.push(instructions::Inst_Set::INST_DIV { _pad: Pad::Padding });
                }
                ParseValue::PRINT { token } => {
                    code.push(instructions::Inst_Set::INST_PRINT { _pad: Pad::Padding });
                }
                ParseValue::ZJMP { token, value } => {
                    let Literal::INT(a) = value;
                    code.push(instructions::Inst_Set::INST_ZJMP { value: a as u32 });
                }
                ParseValue::NZJMP { token, value } => {
                    let Literal::INT(a) = value;
                    code.push(instructions::Inst_Set::INST_NZJMP { value: a as u32 });
                }
                ParseValue::JP { token, value } => {
                    let Literal::INT(a) = value;
                    code.push(instructions::Inst_Set::INST_JP { value: a as u32 });
                }
                ParseValue::CMPG { token } => {
                    code.push(instructions::Inst_Set::INST_CMPG { _pad: Pad::Padding });
                }
                ParseValue::CMPL { token } => {
                    code.push(instructions::Inst_Set::INST_CMPL { _pad: Pad::Padding });
                }
                ParseValue::MOD { token } => {
                    code.push(instructions::Inst_Set::INST_MOD { _pad: Pad::Padding });
                }
                ParseValue::CMPGE { token } => {
                    code.push(instructions::Inst_Set::INST_CMPGE { _pad: Pad::Padding });
                }
                ParseValue::CMPLE { token } => {
                    code.push(instructions::Inst_Set::INST_CMPLE { _pad: Pad::Padding });
                }
                ParseValue::NOP { token } => {
                    code.push(instructions::Inst_Set::INST_NOP { _pad: Pad::Padding });
                }
                ParseValue::HALT { token } => {
                    code.push(instructions::Inst_Set::INST_HALT { _pad: Pad::Padding });
                }
                ParseValue::INDUP { token, value } => {
                    let Literal::INT(a) = value;
                    code.push(instructions::Inst_Set::INST_INDUP { value: a as u32 });
                }
                ParseValue::ISWAP { token, value } => {
                    let Literal::INT(a) = value;
                    code.push(instructions::Inst_Set::INST_ISWAP { value: a as u32 });
                }
                ParseValue::EOF => {}
            }
        }
        Self::write_to_file(file_name, code.as_mut_slice());
    }

    fn write_to_file(path: &str, program: &mut [Inst_Set]) {
        let mut file = File::create(path).unwrap();
        let bytes: &[u8] = bytemuck::must_cast_slice(program);
        // println!("{:?}",bytes);
        file.write_all(bytes).unwrap();
    }
}
