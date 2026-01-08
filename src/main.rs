mod cpu;
mod display_manager;
mod frame_buffer;
mod keypad;
mod memory;

use cpu::cpu::CPU;
use display_manager::display_manager::DisplayManager;
use frame_buffer::frame_buffer::FrameBuffer;
use keypad::keypad::Keypad;
use memory::memory::Memory;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

struct App {
    display_manager: Option<DisplayManager>,
    cpu: CPU,
    memory: Memory,
    frame_buffer: FrameBuffer,
    keypad: Keypad,
}

impl App {
    fn new(rom_path: &str) -> Self {
        let mut memory = Memory::new();
        let rom = std::fs::read(rom_path).unwrap();
        memory.load_rom(&rom);

        App {
            display_manager: None,
            cpu: CPU::new(),
            memory,
            frame_buffer: FrameBuffer::new(),
            keypad: Keypad::new(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.display_manager = Some(DisplayManager::new(event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                const CYCLES_PER_FRAME: usize = 10;

                for _ in 0..CYCLES_PER_FRAME {
                    let opcode = self.cpu.fetch(&self.memory);
                    self.cpu.decode_and_execute(
                        opcode,
                        &mut self.memory,
                        &mut self.frame_buffer,
                        &self.keypad,
                    );
                }

                self.cpu.decrement_timer();

                if let Some(dm) = &mut self.display_manager {
                    dm.render(&self.frame_buffer);
                    dm.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.keypad
                    .handle_key_event(event.physical_key, event.state.is_pressed());
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    // let mut app = App::new("IBM Logo.ch8");
    // let mut app = App::new("test_opcode.ch8");
    let mut app = App::new("./c8games/TETRIS");
    // let mut app = App::new("./c8games/TICTAC");
    event_loop.run_app(&mut app).unwrap();
}
