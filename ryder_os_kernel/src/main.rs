#![no_std]
#![no_main]

use core::fmt::Write;
use core::hint::spin_loop;

pub(crate) mod framebuffer;
pub mod console;
pub mod serial;

use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};
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

    writeln!(console_writer, "Hello GeoStorm").unwrap();
    writeln!(console_writer, "RyderOS is initiated").unwrap();
    writeln!(console_writer, "Ryder wrote me!").unwrap();
    writeln!(console_writer, "Have some other characters").unwrap();
    writeln!(console_writer, "Decimal: {}", 12345).unwrap();
    writeln!(console_writer, "Hex: {:#x}", 0xdead_beef_u64).unwrap();
    writeln!(console_writer, "Binary: {:#b}", 42).unwrap();
    writeln!(
        console_writer,
        "Framebuffer: {}x{}",
        console_writer.width(),
        console_writer.height(),
    ).unwrap();

    loop { spin_loop() }
}

/// Panic handler, called on panic. Since RyderOS needs to run on BareMetal (No underlying OS)
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