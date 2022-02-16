use uart_16550::MmioSerialPort;

use core::fmt::Arguments;
use core::fmt::Write;

use crate::memlayout::UART_BASE_ADDR;

static mut UART_HNDL: Option<MmioSerialPort> = None;

pub fn init() {
    unsafe {
        UART_HNDL = Some(MmioSerialPort::new(UART_BASE_ADDR));
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