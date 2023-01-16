/* struction for all opcode decodings  */
/* refer to this pdf page 104: https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf  */

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
    if opcode == 0x33 {
        ret = RType;
    }
    else if opcode == 0x03 {
        ret = IType;
    }
    else if opcode == 0x13 {
        ret = IType;
    }
    else if opcode == 0x37 {

    }
    
    else if opcode == 0x17 {

    }

    else if opcode == 0x67 {

    }

    
    return ret;
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

}

#[derive(Debug)]
pub struct ITypeInst {
    imm: u16,   /* 12 bits */
    rs1: u8,    /* 5 bits */
    func3: u8,  /* 3 bits */
    rd: u8,     /* 5 bits */
    opcode: u8, /* 7 bits */
}

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
}

#[derive(Debug)]
pub struct UTypeInst {
    imm: u32,       /* 20 bits */
    rd: u8,         /* 5 bits  */ 
    opcode: u8,     /* 7 bits  */
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

}


