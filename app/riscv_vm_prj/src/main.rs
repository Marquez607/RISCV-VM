pub mod memory;
use memory::*;
use vuart:: *;

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

}
    // dump_test();
    // read_write_test();
fn main() {

    let mut uart: Uart = Uart::new();
    let mut count: u8 = 0;

    loop {
        uart.cpu_write_tx_fifo(count);
        count += 1;
        count %= 8;
    }
}
