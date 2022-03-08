use spin::Mutex;
use lazy_static::lazy_static;
use uart_16550::MmioSerialPort;

use core::fmt::Arguments;
use core::fmt::Write;

use crate::uart_print;

lazy_static! {
    static ref UART_HNDL: Mutex<Option<MmioSerialPort>> = {
        Mutex::new(None)
    };
}

pub fn init(uart_base: usize) {
    // Safety: external UART crate
    *UART_HNDL.lock() = Some(unsafe { MmioSerialPort::new(uart_base) });
        
    UART_HNDL
        .lock()
        .as_mut()
        .unwrap()
        .init();
    uart_print!("UART initialized.\n");
}

/// Print simple messages via UART 
pub fn write_str(msg: &str) {
    UART_HNDL
        .lock()
        .as_mut()
        .unwrap()
        .write_str(msg)
        .unwrap();
}

/// Print complex messages via UART, using format_args
pub fn write_fmt(args: Arguments) {
    UART_HNDL
        .lock()
        .as_mut()
        .unwrap()
        .write_fmt(args)
        .unwrap();
}

pub fn get_char() -> char {
    UART_HNDL
        .lock()
        .as_mut()
        .unwrap()
        .receive() as char
}