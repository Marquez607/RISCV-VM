/*
 * name: memory.rs
 * desc: sytem for loading .hex containing riscv program and other memory this will dump
 *       the file to an internal datastructure that'll simulate the CPU memory
 * 
 * Note: note, the .hex files are text files useful for debuge whereas .bin are the actual binary files
 *       this library will support both formats
 * 
 */

use std::{
    fs::{File},
    io::{Read, BufRead, BufReader},
};
use std::fs;
use crate::vuart::*;

#[derive(Debug)]
pub struct Memory {
    filename: String,
    mem: Vec<u8>, /* byte vector */
    size: u64,    /* size of memory, grabbed from vector */
    is_little_endian: bool, /* default = true */
    uart: Uart,
}

/* set to some giant address */
#[derive(Debug)]
enum PeripheralMap {
    UART_FIFO_RX = 0x7000000,    /* read only  */
    UART_FIFO_TX,                /* write only */
    UART_FLAGS,                  /* read only  */
    INVALID,                     /* failure    */
}

impl PeripheralMap {
    pub const BASE: PeripheralMap  = PeripheralMap::UART_FIFO_RX; /* base address, same as first peripheral */
}


impl Memory {
    /* constructor: return blank string and blank vector*/
    pub fn new() -> Memory {
        return Memory {
            filename: String::new(),
            mem: Vec::new(),
            size: 0,
            is_little_endian: true,
            uart: Uart::new(),
        };
    }

    pub fn get_size(&mut self) -> u64 {
        return self.size;
    }

    pub fn make_little_endian(&mut self) {
        self.is_little_endian = true;
    }

    pub fn make_big_endian(&mut self){
        self.is_little_endian = false;
    }
    
    /* check if provided addr is a peripheral mapping */
    pub fn check_peripheral(&mut self, addr: u64 ) -> bool {
        if addr >= PeripheralMap::INVALID as u64{
            return false;
        }

        if addr < PeripheralMap::BASE as u64{
            return false;
        }
        return true;
    }

    /* peripheral handler */
    pub fn peripheral_read(&mut self, addr: u64) -> u8 {
        match addr{
            addr if addr == PeripheralMap::UART_FIFO_RX as u64 => self.uart.cpu_read_rx_fifo(),
            addr if addr == PeripheralMap::UART_FIFO_TX as u64 => 0,
            addr if addr == PeripheralMap::UART_FLAGS as u64 => self.uart.cpu_get_flags(),
            _ => return 0,
        }
    }

    pub fn peripheral_write(&mut self, addr: u64, data: u8) {
        match addr{
            addr if addr == PeripheralMap::UART_FIFO_RX as u64 => return,
            addr if addr == PeripheralMap::UART_FIFO_TX as u64 => self.uart.cpu_write_tx_fifo(data), 
            addr if addr == PeripheralMap::UART_FLAGS as u64 => return,
            _ => return,
        }
    }

    /*
     * name: conv32to8 
     * desc: converts a 32 bit number to a vector of 8 bits
     *       implemented so that we could swap endianness if needed
     */
    pub fn conv32to8(&mut self, x:u32) -> Vec<u8> {
        let mut res = Vec::new();
        if self.is_little_endian {
            for i in 0..4{
                let temp: u8 = ((x >> (i*8)) & 0xFF).try_into().unwrap();
                res.push(temp);
            }
        }
        else { /* is big endian */
            for i in (0..4).rev(){
                let temp: u8 = ((x >> (i*8)) & 0xFF).try_into().unwrap();
                res.push(temp);
            }
        }
        return res;
    }

    pub fn conv8to32(&mut self, x: Vec<u8>) -> u32 {
        let mut res: u32 = 0;

        for i in 0..4 {
            if self.is_little_endian {
                let temp: u32 = x[i] as u32;
                res |= temp << (i*8);
            }
            else {
                let temp: u32 = (x[i] as u32) << 24;
                res |= temp >> (i*8);         
            }
        }
        return res;
    }

    /*
     * name: load_from_text
     * params:
     *  self -> instance of struct
     *  file -> reference to input file 
     * 
     * NOTE: filename must have one 32 bit value per line in hex format
     * 
     */
    pub fn load_from_text(&mut self, infile: &str) -> Result<(),()> {
        self.filename = infile.to_string();
        let file = File::open(infile);
        let file: File = match file {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        self.mem = Vec::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {

            /* this shouldn't panic */
            let byte_str = line.as_ref().unwrap(); 
            
            /* convert hex string to byte vector */
            let num:u32 = match u32::from_str_radix(byte_str,16) {
                Ok(n) => n,
                Err(_) => { 
                    println!("Error: Encountered non-hex number in {}",infile);
                    return Err(());
                }
            };

            let num_vec:Vec<u8> = self.conv32to8(num);
            for x in num_vec{
                self.mem.push(x);
            }
        }

        //if this panics, I'll cry
        self.size = self.mem.len().try_into().unwrap();
        println!("successfully loaded {}",self.filename);
        return Ok(());
    }
    /*
     * name: load_from_text
     * params:
     *  self -> instance of struct
     *  file -> reference to input file 
     * 
     * NOTE: the application expects this to be a compiled riscV binary
     * 
     */
    pub fn load_from_bin(&mut self,infile: &str) -> Result<(),()>{
        self.filename = infile.to_string();
        let file = File::open(infile);
        let mut file: File = match file {
            Ok(f) => f,
            Err(_) => return Err(()),
        };

        let metadata = fs::metadata(&infile).expect("Unable to read metadata");
        self.mem = vec![0;metadata.len() as usize];
        match file.read_exact(&mut self.mem){
            Ok(_) => {
                self.size = self.mem.len().try_into().unwrap();
                return Ok(());
            }
            Err(_) => {
                println!("Error Reading Binary File");
                return Err(());
            }
        }
    }
    
    /* accept address pointing to 8 bit value */
    pub fn read_8bit(&mut self, addr: u64) -> u8 {
        if self.check_peripheral(addr){
            return self.peripheral_read(addr);
        }

        let addr: usize = addr.try_into().unwrap();
        return self.mem[addr];
    }

    /* accept address pointer to 8 bit value */
    pub fn read_32bit(&mut self, addr: u64) -> u32 {
        // also convert to 8 bit address
        let addr: usize = addr as usize;
        let mut slice: Vec<u8> = Vec::new();

        for i in 0..4{
            slice.push(self.read_8bit((addr+i) as u64));
        }

        let res: u32 = self.conv8to32(slice);
        return res; 
    }

    /* accept address pointing to 8 bit value */
    pub fn write_8bit(&mut self, addr: u64, data: u8) {
        if self.check_peripheral(addr){
            return self.peripheral_write(addr,data);
        }
        let addr: usize = addr.try_into().unwrap();
        self.mem[addr] = data;
    }
    
    /* accept address pointer to 8 bit value */
    pub fn write_32bit(&mut self, addr: u64, data: u32) {
        /* convert to 8 bit address */
        let mut addr: usize = addr as usize;
        let bytes:Vec<u8> = self.conv32to8(data);
        for byte in bytes {
            self.write_8bit(addr as u64,byte);
            addr+=1;
        }
    }

    // /* dump memory contents to file */
    // pub fn debug_mem_dump(&mut self, outfile: &String) {
    //     println!("Dumping to {}",outfile);
    //     for addr in &self.mem {
    //         println!("{:02x}",addr);
    //     }
    // }

    /* print current memory to outfile for debug purpose */
    pub fn debug_print_mem_dump(&mut self) {
        let num_words: u64 = self.get_size()/4;
        for i in 0..num_words{
            println!("{:08x}",self.read_32bit(i));
        }
    }
}