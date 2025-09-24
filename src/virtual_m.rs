use crate::instructions::Inst_Set;
pub struct Vm {
    stack: [i32; 1024],
    sp: usize,
    ip: usize,
    instructions: Vec<Inst_Set>,
}

impl Default for Vm {
    fn default() -> Self {
        Vm {
            stack: [0; 1024],
            sp: 0,
            instructions: vec![],
            ip: 0,
        }
    }
}

impl Vm {
    pub fn copy_ins(&mut self, ins: &[Inst_Set]) {
        self.instructions = ins.to_vec();
    }
    pub fn dup(&mut self, index: usize) {
        if index >= self.sp {
            eprint!("ERROR: Index out of range");
        }
        let elem = self.stack[index];
        self.push(elem);
    }
    pub fn swap(&mut self, index: usize) {
        if index >= self.sp {
            eprint!("ERROR: Index out of range");
        }
        let last_index = self.sp - 1;
        self.stack.swap(index, last_index);
    }

    pub fn start(&mut self) {
        while self.ip < self.instructions.len() {
            println!("{:?}",&self.stack[0..self.sp]);

            let i = &self.instructions[self.ip];
            //println!("{:?}",i);
            match i {
                Inst_Set::INST_INDUP { value } => {
                    self.dup(*value as usize);
                }
                Inst_Set::INST_ISWAP { value } => {
                    self.swap(*value as usize);
                }
                Inst_Set::INST_NOP { _pad } => {
                    // nothing
                }
                Inst_Set::INST_HALT { _pad } => {
                    self.ip = self.instructions.len();
                }
                Inst_Set::INST_MOD { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a % b);
                }
                Inst_Set::INST_CMPGE { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b);
                    self.push(a);
                    if a >= b {
                        self.push(1);
                    } else {
                        self.push(0);
                    }
                }
                Inst_Set::INST_CMPLE { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b);
                    self.push(a);
                    if a <= b {
                        self.push(1);
                    } else {
                        self.push(0);
                    }
                }
                Inst_Set::INST_CMPG { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b);
                    self.push(a);
                    if a > b {
                        self.push(1);
                    } else {
                        self.push(0);
                    }
                }
                Inst_Set::INST_CMPL { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b);
                    self.push(a);
                    if a < b {
                        self.push(1);
                    } else {
                        self.push(0);
                    }
                }
                Inst_Set::INST_NZJMP { value } => {
                    let ins = *value as usize;
                    assert!(ins < self.instructions.len(), "jump out of bound");
                    let a = self.pop();
                    if a != 0 {
                        self.ip = ins - 1;
                    }
                }
                Inst_Set::INST_ZJMP { value } => {
                    let ins = *value as usize;
                    assert!(ins < self.instructions.len(), "jump out of bound");
                    let a = self.pop();
                    if a == 0 {
                        self.ip = ins - 1;
                    }
                }
                Inst_Set::INST_JP { value } => {
                    let ins = *value as usize;
                    assert!(ins < self.instructions.len(), "jump out of bound");
                    self.ip = ins - 1;
                }
                Inst_Set::INST_CMPNE { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b);
                    self.push(a);
                    if a != b {
                        self.push(1);
                    } else {
                        self.push(0);
                    }
                }
                Inst_Set::INST_CMPE { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b);
                    self.push(a);
                    if a == b {
                        self.push(1);
                    } else {
                        self.push(0);
                    }
                }
                Inst_Set::INST_SWAP { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a);
                    self.push(b);
                }
                Inst_Set::INST_DUP { _pad } => {
                    let a = self.pop();
                    self.push(a);
                    self.push(a);
                }
                Inst_Set::INST_PUSH { value } => {
                    self.push(*value);
                }
                Inst_Set::INST_POP { _pad } => {
                    self.pop();
                }
                Inst_Set::INST_ADD { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    //println!("debug :- {a},{b}");
                    self.push(a + b);
                }
                Inst_Set::INST_SUB { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a - b);
                }
                Inst_Set::INST_MUL { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a * b);
                }
                Inst_Set::INST_DIV { _pad } => {
                    let a = self.pop();
                    let b = self.pop();
                    if b == 0 {
                        panic!("cannot div by zero");
                    }
                    self.push(a / b);
                }
                Inst_Set::INST_PRINT { _pad } => {
                    println!("{}", self.pop());
                }
            }
            self.ip += 1;
        }
    }

    pub fn push(&mut self, value: i32) {
        assert!(self.sp < 1024, "Stack overflow");
        self.stack[self.sp] = value;
        self.sp += 1;
    }
    pub fn pop(&mut self) -> i32 {
        assert!(self.sp > 0, "stack underflow");
        self.sp -= 1;
        self.stack[self.sp]
    }
}
