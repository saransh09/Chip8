# CHIP-8 Emulator

A CHIP-8 emulator written in Rust, featuring a clean modular architecture and real-time graphics rendering.

## What is CHIP-8?

CHIP-8 is an interpreted programming language developed in the mid-1970s for 8-bit microcomputers. It was designed to make game development easier and more accessible. Despite its simplicity, CHIP-8 has become a popular first project for anyone interested in learning about emulator development.

## Features

- Full implementation of all 35 original CHIP-8 instructions
- 64x32 monochrome display with 10x scaling
- 16-key hexadecimal keypad input
- Built-in font set for characters 0-F
- XOR sprite drawing with collision detection

## Architecture

```
src/
├── main.rs                 # Application entry point & event loop
├── cpu/
│   └── cpu.rs              # CPU with registers, stack, and instruction execution
├── memory/
│   └── memory.rs           # 4KB RAM with font data
├── frame_buffer/
│   └── frame_buffer.rs     # 64x32 pixel display buffer
├── display_manager/
│   └── display_manager.rs  # Window management & rendering (winit + pixels)
└── keypad/
    └── keypad.rs           # Input handling & key mapping
```

## Technical Details

### Memory Map

| Address Range | Description |
|---------------|-------------|
| `0x000-0x1FF` | Reserved (interpreter) |
| `0x050-0x09F` | Built-in font set |
| `0x200-0xFFF` | Program ROM & working RAM |

### Registers

- **V0-VF**: 16 general-purpose 8-bit registers (VF doubles as flag register)
- **I**: 16-bit index register for memory operations
- **PC**: 16-bit program counter (starts at `0x200`)
- **Stack**: Call stack for subroutine return addresses
- **Delay Timer**: 8-bit timer that decrements at 60Hz
- **Sound Timer**: 8-bit timer that beeps when non-zero

### Implemented Instructions

| Opcode | Description |
|--------|-------------|
| `00E0` | Clear the display |
| `00EE` | Return from subroutine |
| `1NNN` | Jump to address NNN |
| `2NNN` | Call subroutine at NNN |
| `3XNN` | Skip next instruction if VX == NN |
| `4XNN` | Skip next instruction if VX != NN |
| `5XY0` | Skip next instruction if VX == VY |
| `6XNN` | Set VX = NN |
| `7XNN` | Add NN to VX (no carry flag) |
| `8XY0` | Set VX = VY |
| `8XY1` | Set VX = VX OR VY |
| `8XY2` | Set VX = VX AND VY |
| `8XY3` | Set VX = VX XOR VY |
| `8XY4` | Add VY to VX (VF = carry) |
| `8XY5` | Subtract VY from VX (VF = NOT borrow) |
| `8XY6` | Shift VX right by 1 (VF = LSB) |
| `8XY7` | Set VX = VY - VX (VF = NOT borrow) |
| `8XYE` | Shift VX left by 1 (VF = MSB) |
| `9XY0` | Skip next instruction if VX != VY |
| `ANNN` | Set I = NNN |
| `BNNN` | Jump to NNN + V0 |
| `CXNN` | Set VX = random byte AND NN |
| `DXYN` | Draw N-byte sprite at (VX, VY) |
| `EX9E` | Skip if key VX is pressed |
| `EXA1` | Skip if key VX is not pressed |
| `FX07` | Set VX = delay timer |
| `FX0A` | Wait for key press, store in VX |
| `FX15` | Set delay timer = VX |
| `FX18` | Set sound timer = VX |
| `FX1E` | Add VX to I |
| `FX29` | Set I = sprite location for digit VX |
| `FX33` | Store BCD of VX at I, I+1, I+2 |
| `FX55` | Store V0-VX in memory starting at I |
| `FX65` | Load V0-VX from memory starting at I |

## Keyboard Layout

The original CHIP-8 had a 16-key hexadecimal keypad. This emulator maps it to:

```
CHIP-8 Keypad        Keyboard Mapping
┌───┬───┬───┬───┐    ┌───┬───┬───┬───┐
│ 1 │ 2 │ 3 │ C │    │ 1 │ 2 │ 3 │ 4 │
├───┼───┼───┼───┤    ├───┼───┼───┼───┤
│ 4 │ 5 │ 6 │ D │    │ Q │ W │ E │ R │
├───┼───┼───┼───┤ -> ├───┼───┼───┼───┤
│ 7 │ 8 │ 9 │ E │    │ A │ S │ D │ F │
├───┼───┼───┼───┤    ├───┼───┼───┼───┤
│ A │ 0 │ B │ F │    │ Z │ X │ C │ V │
└───┴───┴───┴───┘    └───┴───┴───┴───┘
```

## Building & Running

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))

### Build

```bash
cargo build --release
```

### Run

Edit `src/main.rs` to specify your ROM file:

```rust
let mut app = App::new("path/to/your/rom.ch8");
```

Then run:

```bash
cargo run --release
```

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| [winit](https://crates.io/crates/winit) | 0.30 | Cross-platform window creation |
| [pixels](https://crates.io/crates/pixels) | 0.15 | Hardware-accelerated pixel buffer |
| [rand](https://crates.io/crates/rand) | 0.9 | Random number generation (CXNN) |

## Test ROMs

The project includes test ROMs:

- `IBM Logo.ch8` - Displays the IBM logo (tests basic drawing)
- `test_opcode.ch8` - Comprehensive opcode test suite

## Design Decisions

This implementation uses **modern/CHIP-48 behavior** for ambiguous instructions:

- **8XY6/8XYE (Shift)**: Shifts VX in place, ignoring VY
- **FX55/FX65 (Store/Load)**: Does not modify I register
- **BNNN (Jump with offset)**: Uses V0 (original behavior)

## Acknowledgments

This emulator was built following the excellent guide by Tobias V. Langhoff:

**[Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)**

A fantastic resource that explains not just *how* to implement CHIP-8, but *why* each component works the way it does.

## License

MIT
