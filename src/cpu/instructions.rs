use std::convert::TryFrom;

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
            _ => Err(format!("Opcode {} not allowed", opcode)),
        }
    }
}
