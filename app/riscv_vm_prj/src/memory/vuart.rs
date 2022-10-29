/*
 * name: vuart.rs
 * desc: vitural uart for communicating with terminal
 */

use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;
use std::io;

#[derive(Debug)]
enum UartRegs {
    UART_FIFO_RX = 0x7000000,      /* read only  */
    UART_FIFO_TX,                  /* write only */
    UART_FLAGS,                    /* read only  */
    INVALID,                          /* failure    */
}

impl UartRegs {

    pub const BASE: UartRegs = UartRegs::UART_FIFO_RX;
}

/* peripheral flags bit masks */
/* has to be packed into a 8 bit value for register semantics */
#[derive(Debug)]
pub enum UartFlagsBm {
    RX_DATA_AVAIL_bm = 0x01,
}

/* will be wrapping this in a mutex for read/write threads */
#[derive(Debug)]
struct UartComp{
    rx_fifo: VecDeque<u8>,
    tx_fifo: VecDeque<u8>,
    flags: u8,
}

impl UartComp {
    /* constructor: empty vectors */
    pub fn new() -> UartComp {
        return UartComp {
            rx_fifo: VecDeque::new(),
            tx_fifo: VecDeque::new(),
            flags: 0, 
        };
    }
}

/* mutex protected */
#[derive(Clone,Debug)]
pub struct Uart{
    uart_arc: Arc<Mutex<UartComp>>, /* above uart component */
}

/* data coming from the cpu */
fn console_tx_thread( mut uart: Uart ) {
    println!("Tx Thread Spawned");
    loop {
        /* just wait for cpu to transmit data */
        let len: usize = uart.ext_check_tx_fifo_len();
        if( len > 0){
            for i in 0..len{
                print!("{}",uart.ext_read_tx_fifo() as char);
            }
        }
    }
}

/* data going to the cpu */
fn console_rx_thread( mut uart: Uart  ) {
    println!("Rx Thread Spawned");
    loop {
        let mut user_input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut user_input);
        //println!("{}",user_input);
        let char_vec: Vec<char> = user_input.chars().collect();
        for c in char_vec {
            uart.ext_write_rx_fifo(c as u8);
        }      
    }
}

/* providing "bare bones" implemenation, ideally, the CPU will probe the UART flag register 
   before attempting to read 
   
   the console_tx_thread will just prob the vector to see if it's not empty
*/
impl Uart {
    pub fn new() -> Uart {

        let this_uart_arc_orig = Arc::new(Mutex::new(UartComp::new()));

        let uart: Uart = Uart {
            uart_arc: Arc::clone( &this_uart_arc_orig ),
        };

        /* fork back ground threads */
        let thread_uart : Uart = uart.clone();
        thread::spawn(move || {
            console_rx_thread( thread_uart );
        });

        /* write thread */
        let thread_uart : Uart = uart.clone();
        thread::spawn(move || {
            console_tx_thread( thread_uart );
        });

        return uart;
    }

    pub fn cpu_get_flags(&mut self) -> u8 {
        let mut uart = self.uart_arc.lock().unwrap();
        return (*uart).flags;
    }

    /* should not be used by the memory module/cpu */
    pub fn ext_check_tx_fifo_len(&mut self) -> usize {
        let mut uart = self.uart_arc.lock().unwrap();
        return (*uart).tx_fifo.len();
    }

    /* should not be used by the memory module/cpu */
    pub fn ext_read_tx_fifo(&mut self) -> u8 {
        let mut uart = self.uart_arc.lock().unwrap();
        if (*uart).tx_fifo.is_empty(){
            println!("ERROR: tx_fifo empty");
            return 0; 
        }
        return (*uart).tx_fifo.pop_front().unwrap();
    }

    /* should not be used by the memory module */
    pub fn ext_write_rx_fifo(&mut self, data: u8){
        let mut uart = self.uart_arc.lock().unwrap();
        if (*uart).rx_fifo.is_empty(){
            /* set this flag for the CPU to read */
            (*uart).flags |= (UartFlagsBm::RX_DATA_AVAIL_bm as u8);
        }
        (*uart).rx_fifo.push_back(data);
    }

    pub fn cpu_write_tx_fifo(&mut self, data: u8){
        let mut uart = self.uart_arc.lock().unwrap();
        (*uart).tx_fifo.push_back(data);
    }

    pub fn cpu_read_rx_fifo(&mut self) -> u8 {
        let mut uart = self.uart_arc.lock().unwrap();
        if (*uart).rx_fifo.is_empty(){
            println!("ERROR: rx_fifo empty");
            return 0; 
        }
        let ret: u8 = (*uart).rx_fifo.pop_front().unwrap();
        
        /* if that was the last available data in the fifo, clear flag for cpu */
        if (*uart).rx_fifo.is_empty(){
            /* clear the data flag */
            (*uart).flags &= !(UartFlagsBm::RX_DATA_AVAIL_bm as u8);
        }

        return ret;

    }
}
