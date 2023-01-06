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

    fn decode(inst : u32) {

    }

    fn decode_r_type(inst : u32) {

    }

    fn decode_i_type(inst : u32) {

    }

    fn decode_s_type(inst : u32) {

    }

    fn decode_b_type(inst : u32) {

    }

    fn decode_u_type(inst : u32) {

    }

    fn decode_j_type(inst : u32) {

    }




}
