use core::fmt::{Arguments, Write};
use spin::Mutex;
use uart_16550::{backend::PioBackend, Config, Uart16550Tty};

static SERIAL_PORT: Mutex<Option<Uart16550Tty<PioBackend>>> = Mutex::new(None);

pub fn init() {
    let serial_port = unsafe {
        Uart16550Tty::new_port(0x3F8, Config::default())
            .expect("failed to initialize COM1")
    };

    *SERIAL_PORT.lock() = Some(serial_port);
}

pub fn _print(arguments: Arguments<'_>) {
    let mut serial_port = SERIAL_PORT.lock();

    if let Some(serial_port) = serial_port.as_mut() {
        serial_port
            .write_fmt(arguments)
            .expect("failed to write to COM1");
    }
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial_port::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::serial_print!("\n")
    };

    ($($arg:tt)*) => {
        $crate::serial_print!("{}\n", format_args!($($arg)*))
    };
}