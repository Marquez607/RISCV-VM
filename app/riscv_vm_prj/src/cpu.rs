/* 
* name: cpu.rs
* desc: implements CPU logic including registers
*  
*/
use crate::memory::*;

#[derive(Debug)]
pub struct Cpu {
    regs: [u32; 32], /* registers implemented via array */
    pc: u128, /* program counter */
    mem: Memory, 
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu {
            regs: [0;32],
            pc: 0,
            mem: Memory::new(),
        };
    }
}
