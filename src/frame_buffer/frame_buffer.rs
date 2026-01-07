use pixels::wgpu::naga::proc::index;

pub struct FrameBuffer {
    buffer: [u8; 64 * 32],
}

impl FrameBuffer {
    pub fn new() -> Self {
        FrameBuffer {
            buffer: [0; 64 * 32 as usize],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * 64 + x
    }

    pub fn clear(&mut self) {
        self.buffer = [0; 64 * 32 as usize]
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.buffer[self.index(x, y)]
    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) -> bool {
        let idx = self.index(x, y);
        let was_on = self.buffer[idx] == 1;
        self.buffer[idx] ^= 1;
        was_on && self.buffer[idx] == 0
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}
