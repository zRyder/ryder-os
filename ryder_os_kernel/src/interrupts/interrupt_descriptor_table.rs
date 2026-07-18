use spin::Once;
use x86_64::structures::idt::InterruptDescriptorTable;
use crate::interrupts::handlers;
use crate::interrupts::pic::InterruptIndex;

static IDT: Once<InterruptDescriptorTable> = Once::new();

pub fn init() {
    IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt[InterruptIndex::Keyboard.as_u8()]
            .set_handler_fn(handlers::keyboard::keyboard_interrupt_handler);

        idt
    }).load();
}