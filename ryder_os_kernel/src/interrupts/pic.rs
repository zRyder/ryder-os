use pic8259::ChainedPics;
use spin::Mutex;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe {
    ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn as_usize(self) -> usize {
        self.as_u8() as usize
    }
}

pub fn init() {
    unsafe {
        let mut pics = PICS.lock();

        pics.initialize();
        // Timer masks and Keyboard enabled
        pics.write_masks(0b1111_1101, 0b1111_1111);
    }
}

pub fn end_of_interrupt(interrupt: InterruptIndex) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(interrupt.as_u8());
    }
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;