#[macro_export]
macro_rules! uart_print {
    ($($arg:tt)*) => {
        $crate::uart::write_fmt(format_args!($($arg)*));
    };
}