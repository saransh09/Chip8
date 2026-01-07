use crate::{
    frame_buffer::frame_buffer::FrameBuffer, keypad::keypad::Keypad, memory::memory::Memory,
};

pub struct CPU {
    v: [u8; 16],     // V0-VF registers
    i: u16,          // Index register
    pc: u16,         // Program register
    stack: Vec<u16>, // Call stack
    delay_timer: u8, // Decrement at 60Hz
    sound_timer: u8, // Decrement at 60Hz, beeps when > 0
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn fetch(&mut self, memory: &Memory) -> u16 {
        let hi = memory.read(self.pc) as u16;
        let lo = memory.read(self.pc + 1) as u16;
        self.pc += 2;
        (hi << 8) | lo
    }

    pub fn decode_and_execute(
        &mut self,
        opcode: u16,
        memory: &mut Memory,
        display: &mut FrameBuffer,
        keypad: &Keypad,
    ) {
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00E0 => display.clear(), // clear display
                    _ => {}
                }
            }
            0x1000 => {
                // Jump instruction
                self.pc = nnn;
            }
            0x6000 => {
                // Set to VX
                self.v[x] = nn;
            }
            0x7000 => {
                // Add to VX
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            0xA000 => {
                // Set I
                self.i = nnn;
            }
            0xD000 => {
                // Draw sprite
                // XOR drawing algorithm
                self.draw_sprite(memory, display, x, y, n);
            }
            _ => {
                println!("Unknown opcode : {:04X}", opcode);
            }
        }
    }

    fn draw_sprite(
        &mut self,
        memory: &Memory,
        display: &mut FrameBuffer,
        x: usize,
        y: usize,
        n: u8,
    ) {
        let x_coord = self.v[x] as usize % 64;
        let y_coord = self.v[y] as usize % 32;
        self.v[0xF] = 0; // Reset collision flag

        for row in 0..n as usize {
            if y_coord + row >= 32 {
                break;
            }

            let sprite_byte = memory.read(self.i + row as u16);

            for bit in 0..8 {
                if x_coord + bit >= 64 {
                    break;
                }

                let sprite_pixel = (sprite_byte >> (7 - bit)) & 1;

                if sprite_pixel == 1 {
                    // Flip pixel
                    let was_on = display.flip_pixel(x_coord + bit, y_coord + row);

                    // Collision: pixel was ON and is now OFF
                    if was_on {
                        self.v[0xF] = 1;
                    }
                }
            }
        }
    }
}
