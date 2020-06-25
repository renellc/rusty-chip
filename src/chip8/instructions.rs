use std::convert::TryFrom;

/// The instructions available on the CHIP-8 CPU. Each enum contains all the necessary information needed to carry out
/// the instruction (memory address, register number, byte value, etc.), but does not perform the instruction itself.
///
/// # Instructions format
/// The instructions on the CHIP-8 emulator fall into three main types: An instruction which contains a register number
/// some byte value, an instruction which contains a memory address, and an instruction which contains 2 register
/// numbers.
///
/// All enums have their information organized like the following (with the exception of
/// Instruction::DisplayClear and Instruction::FlowReturn): If the opcode contains a memory address, then the num will
/// simply have the memory address contained within it. If the opcode contains a register and a byte value, the register
/// value will come first then the byte value follows it. If the opcode contains two registers, then register X will
/// come first, and register Y follows.
///
/// # Enum value types
/// All register values found in an enum of of type `usize`, while byte values are of type `u8`, so it should be able to
/// decipher what value is what based on this alone. `Instruction::FlowCall` contains a `u16` value as it needs to be
/// pushed on to the stack. `Instruction::FlowJump` contains a `usize` value as it will be used to index the memory
/// address array found in the `chip8::CPU` struct.
///
/// # Examples
///
/// `0x1AF0` => `Instruction::FlowJump(0xAF0)`,
///
/// `0x74F2` => `Instruction::ConstVxAddNN(0x4, 0xF2)`,
///
/// `0x8720` => `Instruction::AssignVxVy(0x7, 0x2)`
///
/// See https://en.wikipedia.org/wiki/CHIP-8#Opcode_table for more information, as well as verification that these
/// opcodes are being decoded correctly.
#[derive(Debug)]
pub enum Instruction {
    /// Clears the display.
    /// Opcode: 00E0
    DisplayClear,

    /// Returns from a subroutine.
    /// Opcode: 00EE
    FlowReturn,

    /// Jumps to address NNN.
    /// Opcode: 1NNN
    FlowJump(usize),

    /// Calls a subroutine at address NNN.
    /// Opcode: 2NNN
    FlowCall(u16),

    /// Skips the next instruction if Vx equals NN.
    /// Opcode: 3XNN
    CondVxNNEq(usize, u8),

    /// Skips the next instruction if Vx does not equal NN.
    /// Opcode: 4XNN
    CondVxNNNeq(usize, u8),

    /// Skips the next instruction if Vx equals Vy.
    /// Opcode: 5XY0
    CondVxVyEq(usize, usize),

    /// Sets Vx to NN
    /// Opcode: 6XNN
    ConstVxNN(usize, u8),

    /// Adds NN to Vx (carry flag unchanged).
    /// Opcode: 7XNN
    ConstVxAddNN(usize, u8),

    /// Sets Vx to the value of Vy.
    /// Opcode: 8XY0
    AssignVxVy(usize, usize),

    /// Sets Vx to (Vx | Vy).
    /// Opcode: 8XY1
    BitOpOR(usize, usize),

    /// Sets Vx to (Vx & Vy).
    /// Opcode: 8XY2
    BitOpAND(usize, usize),

    /// Sets Vx to (Vx ^ Vy).
    /// Opcode: 8XY3
    BitOpXOR(usize, usize),

    /// Adds Vy to Vx. VF is set to 1 when a carry occurs, otherwise it is 0.
    /// Opcode: 8XY4
    MathVxVyAdd(usize, usize),

    /// Subtracts Vy from Vx. VF is set to 0 when a borrow occurs, otherwise it is 1.
    /// Opcode: 8XY5
    MathVxVySub(usize, usize),

    /// Stores the least significant bit of Vx in VF and then shifts Vx to the right by 1.
    /// Opcode: 8XY6
    BitOpShiftRight(usize, usize),

    /// Subtracts Vx from Vy and store the result in Vx. VF is set to 0 when a borrow occurs, otherwise it is 1.
    /// Opcode: 8XY7
    MathVyVxSub(usize, usize),

    /// Stores the most significant bit of Vx in VF and then shifts Vx to the left by 1.
    /// Opcode: 8XYE
    BitOpShiftLeft(usize, usize),

    /// Skips the next instruction if Vx does not equals Vy.
    /// Opcode: 9XY0
    CondVxVyNeq(usize, usize),

    /// Sets I to the address NNN.
    /// Opcode: ANNN
    MemSetIAddress(u16),

    /// Jumps to the address NNN plus the value stored in V0.
    /// Opcode: BNNN
    FlowJumpOffsetV0(usize),

    /// Sets Vx to the result of a bitwise operation on a random number (0 - 255) and NN.
    /// Opcode: CXNN
    RandomANDVxNN(usize, u8),

    /// Draws a sprite at coordinate (Vx, Vy) that has a width of 8 pixels and a height of N pixels.
    /// Opcode: DXYN
    DrawSprite(usize, usize, usize),

    /// Skips the next instruction if the key stored in Vx is pressed.
    /// Opcode: EX9E
    KeyOpKeyPressed(usize),

    /// Skips the next instruction if the key stored in Vx is pressed.
    /// Opcode: EXA1
    KeyOpKeyNotPressed(usize),

    /// Sets Vx to the delay timer value.
    /// Opcode: FX07
    DelayTimerSaveVx(usize),

    /// Wait for the next key press, then store it in Vx. Note that this is a blocking operation.
    /// Opcode: FX0A
    KeyOpGetKey(usize),

    /// Sets the delay timer to Vx.
    /// Opcode: FX15
    DelayTimerSetVx(usize),

    /// Sets the sound timer to Vx.
    /// Opcode: FX18
    SoundTimerSetVx(usize),

    /// Adds Vx to register I. VF is not affected.
    /// Opcode: FX1E
    MemAddIVx(usize),

    /// Sets register I to the location of the sprite for the character in Vx.
    /// Opcode: FX29
    MemSetISprite(usize),

    /// Stores the binary-coded decimal (BCD) representation of Vx in (I=BCD(3)), (I+1=BCD(2)), and (I+2=BCD(1)).
    /// Opcode: FX33
    BCDSave(usize),

    /// Stores V0 to Vx in memory starting at address I.
    /// Opcode: FX55
    MemRegisterDump(usize),

    /// Loads V0 to Vx with the values from memory starting at address I.
    /// Opcode: FX65
    MemRegisterLoad(usize),
}

impl Instruction {
    fn get_registers(opcode: u16) -> (usize, usize) {
        let x = (opcode & 0xF00) >> 8;
        let y = (opcode & 0xF0) >> 4;
        (x as usize, y as usize)
    }

    fn get_register_and_byte(opcode: u16) -> (usize, u8) {
        let x = (opcode & 0xF00) >> 8;
        let byte = opcode & 0xFF;
        (x as usize, byte as u8)
    }

    fn get_address(opcode: u16) -> u16 {
        opcode & 0xFFF
    }
}

impl TryFrom<u16> for Instruction {
    type Error = String;

    fn try_from(opcode: u16) -> Result<Self, Self::Error> {
        if opcode == 0x00E0 {
            return Ok(Instruction::DisplayClear);
        } else if opcode == 0x00EE {
            return Ok(Instruction::FlowReturn);
        }

        let region = opcode & 0xF000;
        match region {
            0x1000 => {
                let addr = (opcode & 0xFFF) as usize;
                Ok(Instruction::FlowJump(addr))
            }
            0x2000 => {
                let addr = opcode & 0xFFF;
                Ok(Instruction::FlowCall(addr))
            }
            0x3000 => {
                let (x, byte) = Instruction::get_register_and_byte(opcode);
                Ok(Instruction::CondVxNNEq(x, byte))
            }
            0x4000 => {
                let (x, byte) = Instruction::get_register_and_byte(opcode);
                Ok(Instruction::CondVxNNNeq(x, byte))
            }
            0x5000 => {
                let (x, y) = Instruction::get_registers(opcode);
                Ok(Instruction::CondVxVyEq(x, y))
            }
            0x6000 => {
                let (x, byte) = Instruction::get_register_and_byte(opcode);
                Ok(Instruction::ConstVxNN(x, byte))
            }
            0x7000 => {
                let (x, byte) = Instruction::get_register_and_byte(opcode);
                Ok(Instruction::ConstVxAddNN(x, byte))
            }
            0x8000 => {
                let (x, y) = Instruction::get_registers(opcode);
                match opcode & 0xF {
                    0x0 => Ok(Instruction::AssignVxVy(x, y)),
                    0x1 => Ok(Instruction::BitOpOR(x, y)),
                    0x2 => Ok(Instruction::BitOpAND(x, y)),
                    0x3 => Ok(Instruction::BitOpXOR(x, y)),
                    0x4 => Ok(Instruction::MathVxVyAdd(x, y)),
                    0x5 => Ok(Instruction::MathVxVySub(x, y)),
                    0x6 => Ok(Instruction::BitOpShiftRight(x, y)),
                    0x7 => Ok(Instruction::MathVyVxSub(x, y)),
                    0xE => Ok(Instruction::BitOpShiftLeft(x, y)),
                    _ => Err(format!("Opcode {} not allowed", opcode)),
                }
            }
            0x9000 => {
                let (x, y) = Instruction::get_registers(opcode);
                Ok(Instruction::CondVxVyNeq(x, y))
            }
            0xA000 => Ok(Instruction::MemSetIAddress(Instruction::get_address(
                opcode,
            ))),
            0xB000 => Ok(Instruction::FlowJumpOffsetV0(
                Instruction::get_address(opcode) as usize,
            )),
            0xC000 => {
                let (x, byte) = Instruction::get_register_and_byte(opcode);
                Ok(Instruction::RandomANDVxNN(x, byte))
            }
            0xD000 => {
                let (x, y) = Instruction::get_registers(opcode);
                let height = opcode & 0xF;
                Ok(Instruction::DrawSprite(x, y, height as usize))
            }
            0xE000 => {
                let register = ((opcode & 0xF00) >> 8) as usize;
                match opcode & 0xFF {
                    0x9E => Ok(Instruction::KeyOpKeyPressed(register)),
                    0xA1 => Ok(Instruction::KeyOpKeyNotPressed(register)),
                    _ => Err(format!("Opcode {} not allowed", opcode)),
                }
            }
            0xF000 => {
                let register = ((opcode & 0xF00) >> 8) as usize;
                match opcode & 0xFF {
                    0x07 => Ok(Instruction::DelayTimerSaveVx(register)),
                    0x0A => Ok(Instruction::KeyOpGetKey(register)),
                    0x15 => Ok(Instruction::DelayTimerSetVx(register)),
                    0x18 => Ok(Instruction::SoundTimerSetVx(register)),
                    0x1E => Ok(Instruction::MemAddIVx(register)),
                    0x29 => Ok(Instruction::MemSetISprite(register)),
                    0x33 => Ok(Instruction::BCDSave(register)),
                    0x55 => Ok(Instruction::MemRegisterDump(register)),
                    0x65 => Ok(Instruction::MemRegisterLoad(register)),
                    _ => Err(format!("Opcode {} not allowed", opcode)),
                }
            }
            _ => Err(format!("Opcode {} not allowed", opcode)),
        }
    }
}
