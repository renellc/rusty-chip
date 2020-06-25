const CHIP8_STACK_SIZE: usize = 16;

/// Represents the stack in the CHIP-8.
pub struct Stack {
    stack: [u16; CHIP8_STACK_SIZE],
    stack_pointer: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: [0; CHIP8_STACK_SIZE],
            stack_pointer: 0,
        }
    }

    /// Pushes a new memory address onto the stack and increases the stack pointer.
    pub fn push(&mut self, addr: u16) {
        self.stack[self.stack_pointer] = addr;
        self.stack_pointer += 1;
    }

    /// Decreases the stack pointer.
    pub fn pop(&mut self) {
        self.stack_pointer -= 1;
    }

    pub fn peek(&self) -> u16 {
        self.stack[self.stack_pointer]
    }
}
