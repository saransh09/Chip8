use pixels::Pixels;

pub struct DisplayManager {
    width: u32,
    height: u32,
    buffer: Vec<Vec<u8>>,
}

impl DisplayManager {
    pub fn new(width: u32, height: u32) -> Self {
        DisplayManager {
            width,
            height,
            buffer: vec![vec![0; 64]; 32],
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![vec![0; 64]; 32]
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.buffer[x][y]
    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) {
        self.buffer[x][y] = !self.buffer[x][y]
    }

    pub fn draw_to_frame(&mut self, frame: &mut [u8]) {
        unimplemented!()
    }
}
