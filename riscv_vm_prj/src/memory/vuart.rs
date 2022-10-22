/*
 * name: vuart.rs
 * desc: vitural uart for communicating with terminal
 */

use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
enum UartRegs {
    BASE = 0x7000000,
    UART_FIFO_RX = 0x7000000,      /* read only  */
    UART_FIFO_TX,                  /* write only */
    UART_FLAGS,                    /* read only  */
    INVALID,                          /* failure    */
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
    rx_fifo: Vec<u8>,
    tx_fifo: Vec<u8>,
    flags: u8,
}

impl UartComp {
    /* constructor: empty vectors */
    pub fn new() -> UartComp {
        return UartComp {
            rx_fifo: Vec::new(),
            tx_fifo: Vec::new(),
            flags: 0, 
        };
    }
}

/* mutex protected */
#[derive(Debug)]
pub struct Uart{
    uart_arc: Arc<Mutex<UartComp>>, /* above uart component */
}

/* data coming from the cpu */
fn console_tx_thread( uart_arc:  Arc<Mutex<UartComp>> ) {
    println!("Tx Thread Spawned");
}

/* data going to the cpu */
fn console_rx_thread( uart_arc:  Arc<Mutex<UartComp>> ) {
    println!("Rx Thread Spawned");
}

/* providing "bare bones" implemenation, ideally, the CPU will probe the UART flag register 
   before attempting to read 
   
   the console_tx_thread will just prob the vector to see if it's not empty
*/
impl Uart {
    pub fn new() -> Uart {

        let this_uart_arc_orig = Arc::new(Mutex::new(UartComp::new()));

        /* fork back ground threads */
        let this_uart_arc = Arc::clone(&this_uart_arc_orig);
        thread::spawn(move || {
            console_read_thread(this_uart_arc);
        });

        /* write thread */
        let this_uart_arc = Arc::clone(&this_uart_arc_orig);
        thread::spawn(move || {
            console_write_thread(this_uart_arc);
        });

        return Uart {
            uart_arc: Arc::clone(&this_uart_arc_orig),
        };
    }

    pub fn get_flags(&mut self) -> u8 {

        let mut uart = self.uart_arc.lock().unwrap();
        return (*uart).flags;
        
    }

    pub fn write_tx_fifo(&mut self, data: u8){

    }

    /* should not be used by the memory module */
    pub fn read_tx_fifo(&mut self) -> u8 {

    }

    /* should not be used by the memory module */
    pub fn write_rx_fifo(&mut self, data: u8){

    }

    pub fn read_rx_fifo(&mut self) -> u8 {

    }
}