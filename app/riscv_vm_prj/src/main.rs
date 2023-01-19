mod memory;
use memory::*;

mod vuart;
use vuart:: *;

mod cpu;
use cpu::*;

mod idecoder;
use idecoder::*;

mod logging;
use logging::*;

const RX_ADDR: u64   = 0x7000000;
const TX_ADDR: u64   = 0x7000001;
const FLAG_ADDR: u64 = 0x7000002;

fn dump_test() {
    let mut memory: Memory = Memory::new();
    match memory.load_from_text("print_array.c.hex"){
        Ok(_) => println!("Mission Accomplished"),
        Err(_) => println!("Done goofed"),
    }
    memory.debug_print_mem_dump();

    let mut memory: Memory = Memory::new();
    match memory.load_from_bin("print_array.c.bin"){
        Ok(_) => println!("Mission Accomplished"),
        Err(_) => println!("Done goofed"),
    }
    memory.debug_print_mem_dump();   
}

fn read_write_test() {
    let mut memory: Memory = Memory::new();
    match memory.load_from_text("print_array.c.hex"){
        Ok(_) => println!("Mission Accomplished"),
        Err(_) => println!("Done goofed"),
    }

    println!("Memory Size = {}",memory.get_size());

    let mut tval32 : u32 = memory.read_32bit(0);
    println!("read_32bit : val={:08x}",tval32);
    let mut tval8 : u8;
    for i in 0..4{
        tval8 = memory.read_8bit(i);
        println!("read_8bit: i={} val={:02x}",i,tval8);
    }

    memory.write_32bit(0, 0x25252525);
    tval32 = memory.read_32bit(0);
    println!("read_32bit : val={:08x}",tval32); 

    for i in 0..4{
        memory.write_8bit(i,0x37);
    }

    for i in 0..4{
        tval8 = memory.read_8bit(i);
        println!("read_8bit: i={} val={:02x}",i,tval8);
    }    

    memory.write_8bit(2,0x20);
    memory.write_8bit(3,0x40);

    tval32 = memory.read_32bit(0);
    println!("read_32bit : val={:08x}",tval32); 

    memory.make_big_endian();

    tval32 = memory.read_32bit(0);
    println!("read_32bit : val={:08x}",tval32); 
}

fn uart_test() {

    let mut memory: Memory = Memory::new();
    match memory.load_from_text("print_array.c.hex"){
        Ok(_) => println!("Mission Accomplished"),
        Err(_) => {
            println!("Done goofed"); 
            return;
        }
    }
    println!("Memory Size = {}",memory.get_size());


    loop {
        while( (memory.read_8bit(FLAG_ADDR) ) == 0 ){}
        let data: u8 = memory.read_8bit(RX_ADDR);
        memory.write_8bit(TX_ADDR,data);
        memory.write_8bit(TX_ADDR,'\n' as u8);
    }  

}
    // dump_test();
    // read_write_test();
fn main() {

    uart_test();
}
