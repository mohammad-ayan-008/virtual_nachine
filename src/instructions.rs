use bytemuck::NoUninit;

#[repr(C)]
#[derive(Debug, Clone, Copy, NoUninit)]
pub enum Pad {
    Padding,
}

#[derive(Debug, Clone, Copy, NoUninit)]
#[repr(C)]
#[allow(private_interfaces)]
pub enum Inst_Set {
    INST_PUSH { value: i32 },
    INST_POP { _pad: Pad },
    INST_CMPE { _pad: Pad },
    INST_CMPNE { _pad: Pad },
    INST_DUP { _pad: Pad },
    INST_ADD { _pad: Pad },
    INST_SWAP { _pad: Pad },
    INST_SUB { _pad: Pad },
    INST_MUL { _pad: Pad },
    INST_DIV { _pad: Pad },
    INST_PRINT { _pad: Pad },
    INST_ZJMP { value: i32 },
    INST_NZJMP { value: i32 },
    INST_JP { value: i32 },
    INST_CMPG { _pad: Pad },
    INST_CMPL { _pad: Pad },
    INST_MOD { _pad: Pad },
    INST_CMPGE { _pad: Pad },
    INST_CMPLE { _pad: Pad },
    INST_NOP { _pad: Pad },
    INST_HALT { _pad: Pad },
    INST_INDUP { value: i32 },
    INST_ISWAP { value: i32 },
}
impl TryFrom<u64> for Inst_Set {
    type Error = String;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let val = (value & 0xFFFF) as u32;
        let value = (value >> 32) as i32 ;
        match val {
            0 => Ok(Inst_Set::INST_PUSH {
                value: value as i32,
            }),
            1 => Ok(Inst_Set::INST_POP { _pad: Pad::Padding }),
            2 => Ok(Inst_Set::INST_CMPE { _pad: Pad::Padding }),
            3 => Ok(Inst_Set::INST_CMPNE { _pad: Pad::Padding }),
            4 => Ok(Inst_Set::INST_DUP { _pad: Pad::Padding }),
            5 => Ok(Inst_Set::INST_ADD { _pad: Pad::Padding }),
            6 => Ok(Inst_Set::INST_SWAP { _pad: Pad::Padding }),
            7 => Ok(Inst_Set::INST_SUB { _pad: Pad::Padding }),
            8 => Ok(Inst_Set::INST_MUL { _pad: Pad::Padding }),
            9 => Ok(Inst_Set::INST_DIV { _pad: Pad::Padding }),
            10 => Ok(Inst_Set::INST_PRINT { _pad: Pad::Padding }),
            11 => Ok(Inst_Set::INST_ZJMP { value  }),
            12 => Ok(Inst_Set::INST_NZJMP { value }),
            13 => Ok(Inst_Set::INST_JP { value }),
            14 => Ok(Inst_Set::INST_CMPG { _pad: Pad::Padding }),
            15 => Ok(Inst_Set::INST_CMPL { _pad: Pad::Padding }),
            16 => Ok(Inst_Set::INST_MOD { _pad: Pad::Padding }),
            17 => Ok(Inst_Set::INST_CMPGE { _pad: Pad::Padding }),
            18 => Ok(Inst_Set::INST_CMPLE { _pad: Pad::Padding }),
            19 => Ok(Inst_Set::INST_NOP { _pad: Pad::Padding }),
            20 => Ok(Inst_Set::INST_HALT { _pad: Pad::Padding }),
            21 => Ok(Inst_Set::INST_INDUP { value }),
            22 => Ok(Inst_Set::INST_ISWAP { value }),
            _ => Err("uknown instruction ".to_owned()),
        }
    }
}
