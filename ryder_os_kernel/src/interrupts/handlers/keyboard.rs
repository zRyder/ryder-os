use spin::Mutex;
use x86_64::{
    instructions::port::Port,
    structures::idt::InterruptStackFrame
};
use x86_64::instructions::interrupts;
use crate::interrupts::pic;
use crate::interrupts::pic::InterruptIndex;

#[derive(Debug)]
struct ScanCodeQueue {
    buffer: [u8; SCAN_CODE_QUEUE_CAPACITY],
    read_index: usize,
    write_index: usize,
    length: usize
}

impl ScanCodeQueue {
    const fn new() -> Self {
        Self {
            buffer: [0; SCAN_CODE_QUEUE_CAPACITY],
            read_index: 0,
            write_index: 0,
            length: 0
        }
    }

    fn push(&mut self, scan_code: u8) -> Result<(), ()>{
        if self.length >= SCAN_CODE_QUEUE_CAPACITY {
            return Err(());
        };

        self.buffer[self.write_index] = scan_code;
        self.write_index = (self.write_index + 1) % SCAN_CODE_QUEUE_CAPACITY;
        self.length += 1;

        Ok(())
    }

    fn pop(&mut self) -> Option<u8> {
        if self.length == 0 {
            return None;
        }

        let scan_code = self.buffer[self.read_index];
        self.read_index = (self.read_index + 1) % SCAN_CODE_QUEUE_CAPACITY;
        self.length -= 1;

        Some(scan_code)
    }
}

static SCAN_CODE_QUEUE: Mutex<ScanCodeQueue> = Mutex::new(ScanCodeQueue::new());

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut data_port = Port::<u8>::new(0x60);
    let scan_code = unsafe { data_port.read() };

    // If the queue is full, drop this byte rather than doing
    // expensive work inside the interrupt handler.
    let _ = SCAN_CODE_QUEUE.lock().push(scan_code);

    pic::end_of_interrupt(InterruptIndex::Keyboard);
}

pub fn take_scan_code() -> Option<u8> {
    interrupts::without_interrupts(|| {
        SCAN_CODE_QUEUE.lock().pop()
    })
}

const SCAN_CODE_QUEUE_CAPACITY: usize = 128;