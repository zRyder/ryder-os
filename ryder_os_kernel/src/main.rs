#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::fmt::Write;
use core::hint::spin_loop;

pub mod framebuffer;
pub mod console;
pub mod serial;
pub mod interrupts;
pub mod keyboard;

use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};
use pc_keyboard::{DecodedKey, KeyCode};
use crate::console::console_writer::ConsoleWriter;
use crate::framebuffer::frame_buffer_writer::FrameBufferWriter;
use crate::serial::serial_port;

/// Entry point override for Operating System named `_start` as it is typically the name for most systems.
/// This entry point is marked with 'extern "C"' because we want to use the C calling convention here because
/// at this point, RUST calling conventions would not be specified. `_start` can never return,
/// denoted by "!" in the return type. `#[no_mangle]` macro prevents unique, hash-like names from
/// being assigned to functions since we want the function name to be `_start.`
/// Prints Hello World! to VGA buffer for now

// ↓ this replaces the `_start` function ↓
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial_port::init();
    serial_println!("Serial initialized");
    serial_println!("Booting RyderOS {:?}", boot_info);

    let frame_buffer = boot_info.framebuffer
        .as_mut()
        .expect("expected framebuffer");

    let frame_buffer_writer = FrameBufferWriter::new(frame_buffer);
    let mut console_writer = ConsoleWriter::new(frame_buffer_writer);
    let mut keyboard_decoder = keyboard::KeyboardDecoder::new();

    interrupts::init();
    serial_println!("Interrupts enabled");

    loop {
        if let Some(scan_code) = interrupts::handlers::keyboard::take_scan_code() {
            serial_println!("Keyboard scancode received: {:#04x}", scan_code);

            if let Some(decoded_key) = keyboard_decoder.decode(scan_code) {
                match decoded_key {
                    DecodedKey::Unicode(character) => {
                        console_writer.write_char(character).unwrap();
                    },
                    DecodedKey::RawKey(key) => {
                        match key {
                            KeyCode::Backspace => {
                                console_writer.write_str("BACK").unwrap();
                            },
                            _ => {
                                serial_println!("Unprocessed key input {:?}", key);
                            }
                        }
                    }
                }
            }
        }

        spin_loop();
    }
}

/// Panic handlers, called on panic. Since RyderOS needs to run on BareMetal (No underlying OS)
/// it cannot use the std implementation of `panic_handler`, we must define it ourselves
/// A `panic_handler` can never return, denoted by "!" in the return type.
/// # Arguments
/// * `_info` - [`PanicInfo`](PanicInfo) containing the file and line where the panic happened
/// and optional panic message
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("KERNEL PANIC: {info}");
    loop { spin_loop() }
}

entry_point!(kernel_main);