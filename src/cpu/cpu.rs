use rand::{rngs::ThreadRng, Rng};

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
    rng: ThreadRng,
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
            rng: rand::rng(),
        }
    }

    pub fn decrement_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.delay_timer > 0 {
            self.sound_timer -= 1;
            // TODO: Could add beep here later
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
                    0x00EE => {
                        // return from stack
                        self.pc = self.stack.pop().unwrap()
                    }
                    _ => {
                        println!("Unknown opcode : {:04X}", opcode);
                    }
                }
            }
            0x1000 => {
                // Jump instruction
                self.pc = nnn;
            }
            0x2000 => {
                // Call subroutine at nnn, push current state to stack
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            0x3000 => {
                if self.v[x] == nn {
                    self.pc += 2
                }
            }
            0x4000 => {
                if self.v[x] != nn {
                    self.pc += 2
                }
            }
            0x5000 => {
                if self.v[x] == self.v[y] {
                    self.pc += 2
                }
            }
            0x6000 => {
                // Set to VX
                self.v[x] = nn;
            }
            0x7000 => {
                // Add to VX
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            0x8000 => {
                // Arithmetic operations
                match opcode & 0x000F {
                    0x0000 => self.v[x] = self.v[y],
                    0x0001 => self.v[x] = self.v[x] | self.v[y],
                    0x0002 => self.v[x] = self.v[x] & self.v[y],
                    0x0003 => self.v[x] = self.v[x] ^ self.v[y],
                    0x0004 => match self.v[x].checked_add(self.v[y]) {
                        Some(v_x) => {
                            self.v[0xF] = 0;
                            self.v[x] = v_x;
                        }
                        None => {
                            self.v[0xF] = 1;
                            self.v[x] = self.v[x].wrapping_add(self.v[y]);
                        }
                    },
                    0x0005 => {
                        let vx = self.v[x];
                        let vy = self.v[y];
                        self.v[x] = vx.wrapping_sub(vy);
                        self.v[0xF] = if vx >= vy { 1 } else { 0 };
                    }
                    0x0006 => {
                        let shifted_out = self.v[x] & 0x1;
                        self.v[x] >>= 1;
                        self.v[0xF] = shifted_out;
                    }
                    0x0007 => {
                        let vx = self.v[x];
                        let vy = self.v[y];
                        self.v[x] = vy.wrapping_sub(vx);
                        self.v[0xF] = if vy >= vx { 1 } else { 0 };
                    }
                    0x0009 => {}
                    0x000E => {
                        let shifted_out = (self.v[x] >> 7) & 0x1;
                        self.v[x] <<= 1;
                        self.v[0xF] = shifted_out;
                    }
                    _ => {}
                }
            }
            0x9000 => {
                if self.v[x] != self.v[y] {
                    self.pc += 2
                }
            }
            0xA000 => {
                // Set I
                self.i = nnn;
            }
            0xB000 => {
                self.pc = nnn + self.v[0] as u16;
            }
            0xC000 => {
                self.v[x] = self.rng.random_range(0..=u8::MAX) & nn;
            }
            0xD000 => {
                // Draw sprite
                // XOR drawing algorithm
                self.draw_sprite(memory, display, x, y, n);
            }
            0xE000 => match opcode & 0x00FF {
                0x009E => {
                    if keypad.is_pressed(self.v[x]) {
                        self.pc += 2;
                    }
                }
                0x00A1 => {
                    if !keypad.is_pressed(self.v[x]) {
                        self.pc += 2;
                    }
                }
                _ => {
                    println!("Unknown opcode : {:04X}", opcode);
                }
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => {
                    self.v[x] = self.delay_timer;
                }
                0x000A => {
                    if let Some(key) = keypad.get_pressed_key() {
                        self.v[x] = key;
                    } else {
                        self.pc -= 2;
                    }
                }
                0x0015 => {
                    self.delay_timer = self.v[x];
                }
                0x0018 => {
                    self.sound_timer = self.v[x];
                }
                0x001E => {
                    self.i = self.i.wrapping_add(self.v[x] as u16);
                    if self.i > 0x0FFF {
                        self.v[0xF] = 1;
                    }
                }
                0x0029 => {
                    self.i = memory.get_font_address(self.v[x]);
                }
                0x0033 => {
                    let vx = self.v[x];
                    memory.write(self.i, vx / 100);
                    memory.write(self.i + 1, (vx / 10) % 10);
                    memory.write(self.i + 2, vx % 10);
                }
                0x0055 => {
                    for i in 0..=x {
                        memory.write(self.i + i as u16, self.v[i]);
                    }
                }
                0x0065 => {
                    for i in 0..=x {
                        self.v[i] = memory.read(self.i + i as u16);
                    }
                }
                _ => {
                    println!("Unknown opcode : {:04X}", opcode);
                }
            },
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
