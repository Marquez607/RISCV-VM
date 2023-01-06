

pub enum RTypeOpcodes {

}

pub enum ITypeOpcodes {
    
}

pub enum STypeOpcodes {
    
}

pub enum BTypeOpcodes {
    
}

pub enum UTypeOpcodes {
    
}

pub enum JTypeOpcodes {
    
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

pub struct ITypeInst {
    imm: u16,   /* 12 bits */
    rs1: u8,    /* 5 bits */
    func3: u8,  /* 3 bits */
    rd: u8,     /* 5 bits */
    opcode: u8, /* 7 bits */
}

/* store type instruction */
pub struct STypeInst {
    imm11_5: u8,     /* 7 bits */
    rs2: u8,      /* 5 bits */
    rs1: u8,      /* 5 bits */
    func3: u8,    /* 3 bits */
    imm4_0: u8,     /* 5 bits */
    opcode: u8,   /* 7 bits */
}

/* branch instruction */
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

pub struct UTypeInst {
    imm: u32,       /* 20 bits */
    rd: u8,         /* 5 bits  */ */
    opcode: u8,     /* 7 bits  */
}

pub struct JTypeInst {
    imm20: u8,       /* 1 bit */
    imm10_1: u16,    /* 10 bits */
    imm11: u8,       /* 1 bit */ 
    imm19_12: u8,    /* 8 bits */
    rd: u8,          /* 5 bits */
    opcode: u8,      /* 7 bits */
}