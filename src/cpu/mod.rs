mod instructions;
mod instructions_test;

use instructions::Instruction;
use std::convert::TryFrom;
use std::fs;

pub struct CPU {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
    pc: usize,
    sp: usize,
    i: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            memory: [0; 4096],
            registers: [0; 16],
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            sp: 0,
            i: 0,
        }
    }

    /// Loads a CHIP-8 rom file into the memory.
    pub fn load_rom(&mut self, file_path: &str) {
        let buffer = fs::read(file_path).unwrap();
        for (i, char) in buffer.iter().enumerate() {
            self.memory[self.pc + i] = *char;
        }
    }

    /// Fetches the opcode at the current program counter.
    pub fn fetch_opcode(&self) -> u16 {
        let hi = (self.memory[self.pc] as u16) << 8;
        let lo = self.memory[self.pc + 1] as u16;
        hi | lo
    }

    /// Decodes the current opcode into a readable instruction.
    pub fn decode_opcode(&self, opcode: u16) -> Result<Instruction, String> {
        Instruction::try_from(opcode)
    }

    /// Executes the specified instruction of the CPU.
    pub fn execute_instruction(&mut self) {}

    /// Updates both the sound and delay timers of the CPU.
    pub fn update_timers(&mut self) {}
}
