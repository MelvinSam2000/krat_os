use log::{Log, Record, Level, Metadata, LevelFilter};

use crate::uart_print;

struct KernelLogger;

impl Log for KernelLogger {

    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            uart_print!("[{}] [{}] {}\n", 
                record.level(), 
                // record.module_path().unwrap(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: KernelLogger = KernelLogger;

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
    log::debug!("Logger initialized.");
}