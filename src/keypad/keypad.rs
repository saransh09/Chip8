use winit::keyboard::{KeyCode, PhysicalKey};

pub struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Keypad { keys: [false; 16] }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn press(&mut self, key: u8) {
        self.keys[key as usize] = true
    }

    pub fn release(&mut self, key: u8) {
        self.keys[key as usize] = false
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        for (i, &pressed) in self.keys.iter().enumerate() {
            if pressed {
                return Some(i as u8);
            }
        }
        None
    }

    pub fn handle_key_event(&mut self, key: PhysicalKey, pressed: bool) {
        if let Some(chip8_key) = self.map_key(key) {
            if pressed {
                self.press(chip8_key);
            } else {
                self.release(chip8_key);
            }
        }
    }

    fn map_key(&self, key: PhysicalKey) -> Option<u8> {
        match key {
            PhysicalKey::Code(KeyCode::Digit1) => Some(0x1),
            PhysicalKey::Code(KeyCode::Digit2) => Some(0x2),
            PhysicalKey::Code(KeyCode::Digit3) => Some(0x3),
            PhysicalKey::Code(KeyCode::Digit4) => Some(0xC),
            PhysicalKey::Code(KeyCode::KeyQ) => Some(0x4),
            PhysicalKey::Code(KeyCode::KeyW) => Some(0x5),
            PhysicalKey::Code(KeyCode::KeyE) => Some(0x6),
            PhysicalKey::Code(KeyCode::KeyR) => Some(0xD),
            PhysicalKey::Code(KeyCode::KeyA) => Some(0x7),
            PhysicalKey::Code(KeyCode::KeyS) => Some(0x8),
            PhysicalKey::Code(KeyCode::KeyD) => Some(0x9),
            PhysicalKey::Code(KeyCode::KeyF) => Some(0xE),
            PhysicalKey::Code(KeyCode::KeyZ) => Some(0xA),
            PhysicalKey::Code(KeyCode::KeyX) => Some(0x0),
            PhysicalKey::Code(KeyCode::KeyC) => Some(0xB),
            PhysicalKey::Code(KeyCode::KeyV) => Some(0xF),
            _ => None,
        }
    }
}
