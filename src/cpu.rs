pub struct Chip8 {
    registers_8bit: [u8; 16],
    register_12bit: u16,
    memory: [u8; 4096],
}
