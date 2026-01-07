use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::Window};

use crate::frame_buffer::frame_buffer::FrameBuffer;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCALE: u32 = 10;

pub struct DisplayManager {
    window: &'static Window,
    pixels: Pixels<'static>,
}

impl DisplayManager {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let window_attr = Window::default_attributes()
            .with_title("CHIP-8")
            .with_inner_size(LogicalSize::new(WIDTH * SCALE, HEIGHT * SCALE));
        let window: &'static Window =
            Box::leak(Box::new(event_loop.create_window(window_attr).unwrap()));

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();
        window.request_redraw();

        DisplayManager { window, pixels }
    }

    pub fn render(&mut self, frame_buffer: &FrameBuffer) {
        let frame = self.pixels.frame_mut();
        for (i, pixel) in frame_buffer.buffer().iter().enumerate() {
            let rgba = if *pixel == 1 {
                [0xFF, 0xFF, 0xFF, 0xFF]
            } else {
                [0x00, 0x00, 0x00, 0xFF]
            };

            let offset = i * 4;
            frame[offset..offset + 4].copy_from_slice(&rgba);
        }
        self.pixels.render().unwrap()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
