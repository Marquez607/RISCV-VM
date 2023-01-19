/* file for all opcode decodings  */
/* refer to this pdf page 104: https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf  */
/* desc of each instruction here https://mark.theis.site/riscv/ */

/* INSTRUCTIONS NOT IMPLEMENTED
   mostly needed for implementing an OS-we will assume pure baremetal
   FENCE
   FENCE.I
   ECALL
   EBREAK
   CSR*
*/

use crate::memory::*;
use std::process;

fn print_log( msg: String){
    println!("{}",msg);
}

#[derive(Debug)]
pub enum InstType {
    RType,
    IType,
    SType,
    BType,
    UType,
    JType,
    Invalid,
}

pub fn opcode_to_InstType(inst: u32) -> InstType {
    use InstType::*; 

    let mut ret: InstType = Invalid;
    let opcode = (inst & 0x7F as u32) as u8;

    return match opcode {

        /* R-Types */
        0x33 => RType,

        /* all IType opcodes */
        0x03 => IType, /* load instructions */
        0x13 => IType, /* ALU instructions */
        0x67 => IType, /* JALR - Jump and Link Reg */
        //0x73 => IType, /* Special Instructions/Ignored */

        /* S-Types */
        0x23 => SType, /* store instructions */

        /* B-Types */
        0x63 => BType, /* branches */

        /* U-Types */
        0x17 => UType, /* AUIPC */
        0x37 => UType, /* LUI */

        /* J-Types */
        0x6F => JType, /* JAL */

        /* just defer this to a higher level */
        _ => Invalid,
    }

}

/* decode 32 bit instruction into fields */
#[derive(Debug)]
pub struct RTypeInst {
    func7 : u8, /* 7 bits */
    rs2: u8,    /* 5 bits */
    rs1: u8,    /* 5 bits */
    func3: u8,  /* 3 bits */ 
    rd: u8,     /* 5 bits */
    opcode: u8, /* 7 bits */
}

/* based on the fun3 field */
/* reminder: only one R-Type opcode, easy to handle with one enum */
pub enum RTypeALUFuncSel {
    ADD_SUB = 0x0, /* both add and sub use this func sel */
    SSL     = 0x1,
    SLT     = 0x2,
    SLTU    = 0x3,
    XOR     = 0x4,
    SRL_SRA = 0x5, /* logical shift left/right */
    OR      = 0x6,
    AND     = 0x7,
}

impl RTypeInst {

    pub fn new( inst: u32) -> RTypeInst {
        let mut func7: u8;
        let mut rs2 : u8;
        let mut rs1 : u8;
        let mut func3 : u8; 
        let mut rd : u8;
        let mut opcode : u8;
        
        func7  = ( (inst >> 25) & 0x7F as u32) as u8;
        rs2    = (( inst >> 20) & 0x1F as u32) as u8;
        rs1    = (( inst >> 15) & 0x1F as u32) as u8;
        func3  = (( inst >> 12) & 0x7  as u32) as u8; 
        rd     = (( inst >> 7 ) & 0x1F as u32) as u8; 
        opcode = (inst & 0x7F as u32) as u8;

        return RTypeInst {
            func7: func7, /* 7 bits */
            rs2: rs2,    /* 5 bits */
            rs1: rs1,    /* 5 bits */
            func3: func3,  /* 3 bits */ 
            rd: rd,     /* 5 bits */
            opcode: opcode, /* 7 bits */
        };
    }

    pub fn execute(&mut self,regs: &mut [u32], pc: &mut u128, mem: &mut Memory) {
        /* casting hell */
        let rs1: u32 = regs[self.rs1 as usize];
        let rs2: u32 = regs[self.rs2 as usize];
        let rd: usize = self.rd as usize;
        let func3: u32 = self.func7.into();

        match func3 {
            func3 if func3 == RTypeALUFuncSel::ADD_SUB as u32 => {
                /* subtract */
                if self.func7 != 0 {
                    regs[rd] = (rs1 as i32 - rs2 as i32) as u32;
                }
                /* add */
                else {
                    regs[rd] = (rs1 as i32 + rs2 as i32) as u32;
                }
            }
            /* shift left logical */
            func3 if func3 == RTypeALUFuncSel::SSL as u32 => {
                regs[rd] = rs1 << rs2;
            }
            /* set less than */
            func3 if func3 == RTypeALUFuncSel::SLT as u32 => {
                regs[rd] = ( (rs1 as i32) < (rs2 as i32) ) as u32;
            }
            /* set less than unsigned */
            func3 if func3 == RTypeALUFuncSel::SLTU as u32 => {
                regs[rd] = ((rs1 as u32) < (rs2 as u32)) as u32;
            }
            /* */
            func3 if func3 == RTypeALUFuncSel::XOR as u32 => {
                regs[rd] = rs1 ^ rs2;
            }
            /* shift left or right arithmatic */
            func3 if func3 == RTypeALUFuncSel::SRL_SRA as u32 => {
                /* SRA */
                if self.func7 != 0{
                    regs[rd] = ((rs1 as i32) >> (rs2 as i32)) as u32;
                }
                /* SLA */
                else{
                    regs[rd] = ((rs1 as i32) << (rs2 as i32)) as u32;
                }
            }
            /*  */
            func3 if func3 == RTypeALUFuncSel::OR as u32 => {
                regs[rd] = rs1 | rs2;
            } 
            /*  */
            func3 if func3 == RTypeALUFuncSel::AND as u32 => {
                regs[rd] = rs1 & rs2;
            }

            _ => {
                print_log("Error: RTypeInst execute func3: invalid sel".to_string());
            }

        }

    }
}

#[derive(Debug)]
pub struct ITypeInst {
    imm: u16,   /* 12 bits */
    rs1: u8,    /* 5 bits */
    func3: u8,  /* 3 bits */
    rd: u8,     /* 5 bits */
    opcode: u8, /* 7 bits */
}

/* supports Load,immediate ALU, and JAL*/
pub enum ITypeOpcodes {
    LD   = 0x03, /* load */
    ALU  = 0x13,
    JALR = 0x67,
}

/* ITypeLoadFuncSel */
pub enum ITypeLoadFuncSel {
    LB  = 0x0,
    LH  = 0x1,
    LW  = 0x2,
    LBU = 0x4,
    LHU = 0x5,
}

/* ITypeALUFuncSel */
pub enum ITypeALUFuncSel {
    ADDI  = 0x0,
    SLTI  = 0x2,
    STLIU = 0x3,
    XORI  = 0x4,
    ORI   = 0x6,
    ANDI  = 0x7,
}

/* ITypeJALFuncSel */
// unneeded 

impl ITypeInst {

    pub fn new(inst: u32) -> ITypeInst {

        let mut imm: u16;
        let mut rs1: u8;
        let mut func3: u8;
        let mut rd: u8;
        let mut opcode: u8;

        imm   = (( inst >> 20) & 0xFFF as u32) as u16;
        rs1   = (( inst >> 15) & 0x1F  as u32) as u8;
        func3 = (( inst >> 12) & 07    as u32) as u8;
        rd    = (( inst >> 7 ) & 0x1F  as u32) as u8;
        opcode = (inst & 0x7F as u32) as u8;

        return ITypeInst {
            imm: imm,
            rs1: rs1,
            func3: func3,
            rd: rd,
            opcode: opcode,
        };

    }

    pub fn execute(&mut self,arr: &mut [u32], pc: &mut u128, mem: &mut Memory) {

    }

}

/* store type instruction */
#[derive(Debug)]
pub struct STypeInst {
    imm11_5: u8,     /* 7 bits */
    rs2: u8,         /* 5 bits */
    rs1: u8,         /* 5 bits */
    func3: u8,       /* 3 bits */
    imm4_0: u8,      /* 5 bits */
    opcode: u8,      /* 7 bits */
}

/* only has one associated opcode */
pub enum STypeStoreFuncSel {
    SB = 0x0,
    SH = 0x1,
    SW = 0x2,
}  

impl STypeInst {

    pub fn new(inst: u32) -> STypeInst {

        let mut imm11_5: u8;     /* 7 bits */
        let mut rs2: u8;         /* 5 bits */
        let mut rs1: u8;         /* 5 bits */
        let mut func3: u8;       /* 3 bits */
        let mut imm4_0: u8;      /* 5 bits */
        let mut opcode: u8;      /* 7 bits */

        imm11_5 = (( inst >> 25) & 0x7F as u32) as u8;
        rs2     = (( inst >> 20) & 0x1F as u32) as u8;
        rs1     = (( inst >> 15) & 0x1F as u32) as u8;
        func3   = (( inst >> 12) & 0x7  as u32) as u8;
        imm4_0  = (( inst >> 7 ) & 0x1F as u32) as u8;
        opcode = (inst & 0x7F as u32) as u8;

        return STypeInst {
            imm11_5: imm11_5,
            rs2: rs2,
            rs1: rs1,
            func3: func3,
            imm4_0: imm4_0,
            opcode: opcode,
        };

    }

    pub fn execute(&mut self,arr: &mut [u32], pc: &mut u128, mem: &mut Memory) {

    }

}

/* branch instruction */
#[derive(Debug)]
pub struct BTypeInst {
    imm12: u8,      /* 1 bit */
    imm10_5: u8,    /* 6 bits */
    rs2: u8,        /* 5 bits */
    rs1: u8,        /* 5 bits */
    func3: u8,      /* 3 bits */
    imm4_1: u8,     /* 4 bits */
    imm11: u8,      /* 1 bit */
    opcode: u8,     /* 7 bits */
}

pub enum BTypeBranchFuncSel {
    BEQ = 0x0,
    BNE = 0x1,
    BLT = 0x4,
    BGE = 0x5,
    BLTU = 0x6,
    BGEU = 0x7,
}

impl BTypeInst {

    pub fn new(inst: u32) -> BTypeInst {
        let mut imm12: u8;      /* 1 bit */
        let mut imm10_5: u8;    /* 6 bits */
        let mut rs2: u8;        /* 5 bits */
        let mut rs1: u8;        /* 5 bits */
        let mut func3: u8;      /* 3 bits */
        let mut imm4_1: u8;     /* 4 bits */
        let mut imm11: u8;      /* 1 bit */
        let mut opcode: u8;     /* 7 bits */

        imm12 = (( inst >> 31) & 0x01 as u32) as u8;
        imm10_5 = (( inst >> 25) & 0x3F as u32) as u8;
        rs2 =  (( inst >> 20) & 0x1F as u32) as u8;
        rs1 = (( inst >> 15) & 0x1F as u32) as u8;
        func3 =  (( inst >> 12) & 0x07 as u32) as u8;
        imm4_1 = (( inst >> 8) & 0x0F as u32) as u8;
        imm11 =  (( inst >> 7) & 0x01 as u32) as u8;
        opcode = (inst & 0x7F as u32) as u8;

        return BTypeInst {
            imm12: imm12,
            imm10_5: imm10_5,
            rs2: rs2,
            rs1: rs1,
            func3: func3,
            imm4_1: imm4_1,
            imm11: imm11,
            opcode: opcode,
        };
    }

    pub fn execute(&mut self,arr: &mut [u32], pc: &mut u128, mem: &mut Memory) {

    }
}

#[derive(Debug)]
pub struct UTypeInst {
    imm: u32,       /* 20 bits */
    rd: u8,         /* 5 bits  */ 
    opcode: u8,     /* 7 bits  */
}

pub enum UTypeOpcodes {
    AUIPC = 0x17,
    LUI   = 0x37, 
}

impl UTypeInst {

    pub fn new(inst: u32) -> UTypeInst {
        let mut imm: u32;       /* 20 bits */
        let mut rd: u8;        /* 5 bits  */ 
        let mut opcode: u8;     /* 7 bits  */

        imm    = (( inst >> 12) & 0xFFFFF as u32) as u32;
        rd     = (( inst >> 7) & 0x1F as u32) as u8;
        opcode = (inst & 0x7F as u32) as u8;

        return UTypeInst {
            imm: imm,
            rd: rd,
            opcode: opcode,
        };
    }

    pub fn execute(&mut self,arr: &mut [u32], pc: &mut u128, mem: &mut Memory) {

    }

}

#[derive(Debug)]
pub struct JTypeInst {
    imm20: u8,       /* 1 bit */
    imm10_1: u16,    /* 10 bits */
    imm11: u8,       /* 1 bit */ 
    imm19_12: u8,    /* 8 bits */
    rd: u8,          /* 5 bits */
    opcode: u8,      /* 7 bits */
}

impl JTypeInst {

    pub fn new(inst: u32) -> JTypeInst {
        let mut imm20: u8;
        let mut imm10_1: u16;
        let mut imm11: u8;
        let mut imm19_12: u8;
        let mut rd: u8;
        let mut opcode: u8;

        imm20    = ((inst >> 31) & 0x01 as u32) as u8;
        imm10_1  = ((inst >> 21) & 0x3FF as u32) as u16;
        imm11    = ((inst >> 22) & 0x01 as u32) as u8;
        imm19_12 = ((inst >> 12) & 0x01 as u32) as u8;
        rd       = ((inst >> 7) & 0x1F as u32) as u8;
        opcode   = (inst & 0x7F as u32) as u8;

        return JTypeInst {
            imm20: imm20,
            imm10_1: imm10_1,
            imm11: imm11,
            imm19_12: imm19_12,
            rd: rd,
            opcode: opcode,
        };
    }

    pub fn execute(&mut self,arr: &mut [u32], pc: &mut u128, mem: &mut Memory) {

    }

}


