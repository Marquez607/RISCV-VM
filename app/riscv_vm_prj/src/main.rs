pub mod memory;
use memory::*;

fn main() {

    let mut memory: Memory = Memory::new();
    match memory.load_from_text("print_array.c.hex"){
        Ok(_) => println!("Mission Accomplished"),
        Err(_) => println!("Done goofed"),
    }
    memory.debug_mem_dump(&"hex_test.txt".to_string());


    let mut memory: Memory = Memory::new();
    match memory.load_from_bin("print_array.c.bin"){
        Ok(_) => println!("Mission Accomplished"),
        Err(_) => println!("Done goofed"),
    }
    memory.debug_mem_dump(&"bin_test.txt".to_string());

    //memory.is_little_endian=false;
    //memory.debug_mem_dump(&"Test.txt".to_string());
    
}
