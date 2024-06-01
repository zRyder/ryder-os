#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};

static HELLO: &[u8] = b"Hello World!";

/// Entry point override for Operating System named `_start` as it is typically the name for most systems.
/// This entry point is marked with 'extern "C"' because we want to use the C calling convention here because
/// at this point, RUST calling conventions would not be specified. `_start` can never return,
/// denoted by "!" in the return type. `#[no_mangle]` macro prevents unique, hash-like names from
/// being assigned to functions since we want the function name to be `_start.`
/// Prints Hello World! to VGA buffer for now

// ↓ this replaces the `_start` function ↓
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        for byte in framebuffer.buffer_mut() {
            *byte = 0x90;
        }
    }
    loop {}
}

/// Panic handler, called on panic. Since RyderOS needs to run on BareMetal (No underlying OS)
/// it cannot use the std implementation of `panic_handler`, we must define it ourselves
/// A `panic_handler` can never return, denoted by "!" in the return type.
/// # Arguments
/// * `_info` - [`PanicInfo`](PanicInfo) containing the file and line where the panic happened
/// and optional panic message
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(kernel_main);