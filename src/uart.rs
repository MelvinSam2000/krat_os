use uart_16550::MmioSerialPort;

use core::fmt::Arguments;
use core::fmt::Write;

static mut UART_HNDL: Option<MmioSerialPort> = None;

pub fn init(uart_base: usize) {
    unsafe {
        UART_HNDL = Some(MmioSerialPort::new(uart_base));
        UART_HNDL.as_mut().unwrap().init();
    }
}

/// Print simple messages via UART 
pub fn write_str(msg: &str) {
    unsafe {
        UART_HNDL.as_mut().unwrap().write_str(msg).unwrap();
    }
}

/// Print complex messages via UART, using format_args
pub fn write_fmt(args: Arguments) {
    unsafe {
        UART_HNDL.as_mut().unwrap().write_fmt(args).unwrap();
    }
}

pub fn get_char() -> char {
    unsafe {
        UART_HNDL.as_mut().unwrap().receive() as char
    }
}