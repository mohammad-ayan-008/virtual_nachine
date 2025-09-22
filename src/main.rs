#![allow(non_camel_case_types)]
use crate::instructions::Inst_Set;
use crate::instructions::Pad;
use crate::lexer::Lexer;
use crate::virtual_m::Vm;
use std::fs;
use std::{
    fs::File,
    io::{Read, Write},
    vec,
};
mod instructions;
mod lexer;
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
    let data = fs::read_to_string("test.msm").unwrap();
    let mut lexer = Lexer::read_source(&data); 
    let d = lexer.lexe().unwrap();
    println!("{:?}",d);

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

    write_to_file("test5.tim", &mut program3);
    let mut vm = Vm::default();
    let data = read_from_file("test5.tim");

    vm.copy_ins(&data);
    vm.start();
}

fn write_to_file(path: &str, program: &mut [Inst_Set]) {
    let mut file = File::create(path).unwrap();
    let bytes: &[u8] = bytemuck::must_cast_slice(program);
    // println!("{:?}",bytes);
    file.write_all(bytes).unwrap();
}

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
