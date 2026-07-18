pub mod interrupt_descriptor_table;
pub mod pic;
pub mod handlers;

pub fn init() {
    interrupt_descriptor_table::init();
    pic::init();

    x86_64::instructions::interrupts::enable();
}