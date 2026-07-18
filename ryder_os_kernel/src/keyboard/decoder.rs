use pc_keyboard::{DecodedKey, HandleControl, PS2Keyboard, ScancodeSet1};
use pc_keyboard::layouts::Us104Key;

pub struct KeyboardDecoder {
    keyboard: PS2Keyboard<Us104Key, ScancodeSet1>
}

impl KeyboardDecoder {
    pub const fn new() -> Self {
        Self {
            keyboard: PS2Keyboard::new(
                ScancodeSet1::new(),
                Us104Key,
                HandleControl::Ignore
            )
        }
    }

    pub fn decode(
        &mut self,
        scan_code: u8
    ) -> Option<DecodedKey> {
        let key_event = match self.keyboard.add_byte(scan_code) {
            Ok(Some(key_event)) => key_event,
            Ok(None) => return None,
            // Ignore unknown keypresses
            Err(_) => return None,
        };

        self.keyboard.process_keyevent(key_event)
    }
}