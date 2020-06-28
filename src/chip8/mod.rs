mod display;
mod input;
mod instructions;
mod instructions_test;
mod memory;
mod stack;

use instructions::Instruction;
use rand::Rng;
use std::convert::TryFrom;

pub struct Chip8 {
    memory: memory::Memory,
    stack: stack::Stack,
    registers: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    i: u16,
    rand: rand::rngs::ThreadRng,
    input: input::Input,
    display: display::Display,
}

impl Chip8 {
    pub fn new() -> Self {
        let sdl_ctx = sdl2::init().unwrap();

        Chip8 {
            memory: memory::Memory::new(),
            stack: stack::Stack::new(),
            registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            i: 0,
            rand: rand::thread_rng(),
            input: input::Input::new(&sdl_ctx),
            display: display::Display::new(&sdl_ctx, 10),
        }
    }

    /// Loads a CHIP-8 rom file into the memory.
    pub fn load_rom(&mut self, file_path: &str) {
        self.memory.load_rom(file_path);
    }

    /// Fetches the opcode at the current program counter.
    pub fn fetch_opcode(&self) -> u16 {
        self.memory.fetch_opcode()
    }

    /// Decodes the current opcode into a readable instruction.
    pub fn decode_opcode(&self, opcode: u16) -> Result<Instruction, String> {
        Instruction::try_from(opcode)
    }

    /// Executes the specified instruction of the CPU.
    pub fn execute_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::DisplayClear => {
                self.display.clear_screen();
            }
            Instruction::FlowReturn => {
                self.stack.pop();
                self.memory.jump(self.stack.peek() as usize);
            }
            Instruction::FlowJump(addr) => {
                self.memory.jump(addr);
            }
            Instruction::FlowCall(addr) => {
                self.stack.push(self.memory.get_program_counter() as u16);
                self.memory.jump(addr as usize);
            }
            Instruction::CondVxNNEq(reg, byte) => {
                if self.registers[reg] == byte {
                    self.memory.next_instruction();
                }
            }
            Instruction::CondVxNNNeq(reg, byte) => {
                if self.registers[reg] != byte {
                    self.memory.next_instruction();
                }
            }
            Instruction::CondVxVyEq(x, y) => {
                if self.registers[x] == self.registers[y] {
                    self.memory.next_instruction();
                }
            }
            Instruction::ConstVxNN(reg, byte) => {
                self.registers[reg] = byte;
            }
            Instruction::ConstVxAddNN(reg, byte) => {
                self.registers[reg] += byte;
            }
            Instruction::AssignVxVy(x, y) => {
                self.registers[x] = self.registers[y];
            }
            Instruction::BitOpOR(x, y) => {
                self.registers[x] = self.registers[x] | self.registers[y];
            }
            Instruction::BitOpAND(x, y) => {
                self.registers[x] = self.registers[x] & self.registers[y];
            }
            Instruction::BitOpXOR(x, y) => {
                self.registers[x] = self.registers[x] ^ self.registers[y];
            }
            Instruction::MathVxVyAdd(x, y) => {
                let (res, did_overflow) = self.registers[x].overflowing_add(self.registers[y]);

                if did_overflow {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }

                self.registers[x] = res;
            }
            Instruction::MathVxVySub(x, y) => {
                let (res, did_overflow) = self.registers[x].overflowing_sub(self.registers[y]);

                if did_overflow {
                    self.registers[0xF] = 0;
                } else {
                    self.registers[0xF] = 1;
                }

                self.registers[x] = res;
            }
            Instruction::BitOpShiftRight(x, _) => {
                let lsb = self.registers[x] & 0b0000_0001;
                self.registers[0xF] = lsb;
                self.registers[x] >>= 1;
            }
            Instruction::MathVyVxSub(x, y) => {
                let (res, did_overflow) = self.registers[y].overflowing_sub(self.registers[x]);

                if did_overflow {
                    self.registers[0xF] = 0;
                } else {
                    self.registers[0xF] = 1;
                }

                self.registers[x] = res;
            }
            Instruction::BitOpShiftLeft(x, _) => {
                let msb = self.registers[x] & 0b1000_0000;
                self.registers[0xF] = msb >> 7;
                self.registers[x] <<= 1;
            }
            Instruction::CondVxVyNeq(x, y) => {
                if self.registers[x] == self.registers[y] {
                    self.memory.next_instruction();
                }
            }
            Instruction::MemSetIAddress(addr) => {
                self.i = addr;
            }
            Instruction::FlowJumpOffsetV0(addr) => {
                self.memory.jump(addr + (self.registers[0] as usize));
            }
            Instruction::RandomANDVxNN(reg, byte) => {
                self.registers[reg] = self.rand.gen::<u8>() & byte;
            }
            Instruction::DrawSprite(x, y, height) => {
                let (x_pos, y_pos) = (self.registers[x], self.registers[y]);
                self.registers[0xF] = 0;

                for y in 0..height {
                    let byte = self.memory.get_mem(self.i as usize + y);
                    // We go up to 8 since each row in memory is 8 bits long
                    for x in 0..8 {
                        let pixel_on = byte & (0x80 >> col);
                        if pixel_on {
                            let pixel = if self.display.screen[y_pos][x_pos] { 1 } else { 0 };
                            let collision_occurs = pixel == 0xFFFFFFFF;
                            if collision_occurs {
                                self.registers[0xF] = 1;
                            }
                            self.display.screen[y_pos][x_pos] = pixel ^ 0xFFFFFFFF == 1;
                        }
                    }
                }
            }
            Instruction::KeyOpKeyPressed(reg) => {
                if self.input.is_pressed(self.registers[reg] as usize) {
                    self.memory.next_instruction();
                }
            }
            Instruction::KeyOpKeyNotPressed(reg) => {
                if !self.input.is_pressed(self.registers[reg] as usize) {
                    self.memory.next_instruction();
                }
            }
            Instruction::DelayTimerSaveVx(reg) => {
                self.registers[reg] = self.delay_timer;
            }
            Instruction::KeyOpGetKey(reg) => 'key_wait: loop {
                self.input.get_inputs();
                for i in 0..16 as usize {
                    if self.input.is_pressed(i) {
                        self.registers[reg] = i as u8;
                        break 'key_wait;
                    }
                }
            },
            Instruction::DelayTimerSetVx(reg) => {
                self.delay_timer = self.registers[reg];
            }
            Instruction::SoundTimerSetVx(reg) => {
                self.sound_timer = self.registers[reg];
            }
            Instruction::MemAddIVx(reg) => {
                self.i += self.registers[reg] as u16;
            }
            Instruction::MemSetISprite(_) => {}
            Instruction::BCDSave(reg) => {
                let mut value: u8 = self.registers[reg];

                for offset in (0..3).rev() {
                    self.memory.set_mem((self.i + offset) as usize, value % 10);
                    value /= 10;
                }
            }
            Instruction::MemRegisterDump(reg_end) => {
                for reg in 0..(reg_end + 1) {
                    self.memory
                        .set_mem((self.i + reg as u16) as usize, self.registers[reg]);
                }
            }
            Instruction::MemRegisterLoad(reg_end) => {
                for reg in 0..(reg_end + 1) {
                    self.registers[reg] = self.memory.get_mem((self.i + reg as u16) as usize);
                }
            }
        }
    }

    /// Updates both the sound and delay timers of the CPU.
    pub fn update_timers(&mut self) {}
}
