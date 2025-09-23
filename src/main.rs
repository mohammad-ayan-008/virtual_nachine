#![allow(non_camel_case_types)]
use crate::instructions::Inst_Set;
use crate::instructions::Pad;
use crate::lexer::Lexer;
use crate::virtual_m::Vm;
use std::env::args;
use std::fs;
use std::{
    fs::File,
    io::{Read, Write},
    vec,
};
mod codegen;
mod instructions;
mod lexer;
mod parser;
mod virtual_m;

macro_rules! INS {
    ($val:literal,$name:ident) => {
        Inst_Set::$name { value: $val }
    };
    ($name:ident) => {
        Inst_Set::$name { _pad: Pad::Padding }
    };
}

fn main() {
    let arg: Vec<String> = args().collect();
    if arg.len() > 1 {
        println!("{:?}", arg);
        if &arg[1] == "r" {
            let code = read_from_file(&arg[2]);
            let mut vm = Vm::default();
            vm.copy_ins(&code);
            vm.start();
        } else if &arg[1] == "b" {
            let file_name = &arg[2];
            if !file_name.contains(".tim") {
                panic!("uknown file type");
            }
            let data = fs::read_to_string(file_name).unwrap();
            let mut lexer = Lexer::read_source(&data);
            let mut parser = parser::Parser::new(&mut lexer);
            let codegen = codegen::CodeGen::new(parser.parse());
            let file_name =file_name.replace(".tim", ".msm");
            codegen.generat_(&file_name);
        }
    }
}
/*
fn write_to_file(path: &str, program: &mut [Inst_Set]) {
    let mut file = File::create(path).unwrap();
    let bytes: &[u8] = bytemuck::must_cast_slice(program);
    // println!("{:?}",bytes);
    file.write_all(bytes).unwrap();
}


    let mut program3 = [
        INS!(1, INST_PUSH),
        INS!(2, INST_PUSH),
        INS!(3, INST_PUSH),
        INS!(4, INST_PUSH),
        INS!(0, INST_ISWAP),
        INS!(3, INST_INDUP),
        INS!(INST_PRINT),
        INS!(INST_PRINT),
    ];
*/
fn read_from_file(path: &str) -> Vec<Inst_Set> {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let mut vec_ins = vec![];
    for i in data.chunks_exact(8) {
        let ins = u64::from_ne_bytes(i[..].try_into().unwrap());
        let a = Inst_Set::try_from(ins).unwrap();
        vec_ins.push(a);
    }
    vec_ins
}
