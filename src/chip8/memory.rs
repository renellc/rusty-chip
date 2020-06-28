use std::fs;

const CHIP8_MEM_SIZE: usize = 0x1000;
const CHIP8_MEM_START: usize = 0x200;
const CHIP8_MEM_FONT_START: usize = 0x50;

const CHIP8_FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// Represents the memory inside the CHIP-8. This contains both the memory addresses and the program counter.
pub struct Memory {
    mem: [u8; CHIP8_MEM_SIZE],
    program_counter: usize,
}

impl Memory {
    pub fn new() -> Self {
        let mut mem = Memory {
            mem: [0; CHIP8_MEM_SIZE],
            program_counter: CHIP8_MEM_START,
        };

        // Load fonts into memory
        for (i, byte) in CHIP8_FONT_SET.iter().enumerate() {
            mem.mem[CHIP8_MEM_FONT_START + i] = *byte;
        }

        mem
    }

    pub fn get_program_counter(&self) -> usize {
        self.program_counter
    }

    /// Loads a CHIP-8 rom file into the memory.
    pub fn load_rom(&mut self, rom: &str) {
        let buffer = fs::read(rom).unwrap();
        for (i, char) in buffer.iter().enumerate() {
            self.mem[self.program_counter + i] = *char;
        }
    }

    /// Fetches the next opcode in memory.
    pub fn fetch_opcode(&self) -> u16 {
        let hi = (self.mem[self.program_counter] as u16) << 8;
        let lo = self.mem[self.program_counter + 1] as u16;
        hi | lo
    }

    /// Jumps to the specified memory address.
    pub fn jump(&mut self, addr: usize) {
        self.program_counter = addr;
    }

    /// Skips the next opcode in memory.
    pub fn next_instruction(&mut self) {
        self.program_counter += 2;
    }

    pub fn set_mem(&mut self, addr: usize, value: u8) {
        self.mem[addr] = value;
    }

    pub fn get_mem(&self, addr: usize) -> u8 {
        self.mem[addr]
    }
}
