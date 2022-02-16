use uart_16550::MmioSerialPort;
use lazy_static::lazy_static;
use spin::Mutex;

use core::fmt::Arguments;
use core::fmt::Write;

use crate::memlayout::UART_BASE_ADDR;

lazy_static! {
    static ref UART_HNDL: Mutex<MmioSerialPort> = {
        let mut uart_hndl = unsafe { MmioSerialPort::new(UART_BASE_ADDR) };
        uart_hndl.init();
        Mutex::new(uart_hndl)
    };
}

/// Print simple messages via UART 
pub fn write_str(msg: &str) { 
    UART_HNDL.lock().write_str(msg).unwrap();
}

/// Print complex messages via UART, using format_args
pub fn write_fmt(args: Arguments) {
    UART_HNDL.lock().write_fmt(args).unwrap();
}