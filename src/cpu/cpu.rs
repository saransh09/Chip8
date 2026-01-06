pub struct CPU {
    v: [u8; 16],     // V0-VF registers
    i: u16,          // Index register
    pc: u16,         // Program register
    stack: Vec<u16>, // Call stack
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            v: [0; 16],
            i: 0,
            pc: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}
