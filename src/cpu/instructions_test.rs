#[cfg(test)]
mod instructions_parse_test {
    use crate::cpu::instructions::Instruction;
    use std::convert::TryFrom;

    #[test]
    fn try_into_test_1nnn() {
        let opcode = 0x1FA3;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::FlowJump(addr) = instr {
            assert_eq!(addr, 0xFA3);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }
    }

    #[test]
    fn try_into_test_2nnn() {
        let opcode = 0x2A02;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::FlowCall(addr) = instr {
            assert_eq!(addr, 0xA02);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }
    }

    #[test]
    fn try_into_test_3xnn() {
        let opcode = 0x3B22;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::CondVxNNEq(reg, byte) = instr {
            assert_eq!(reg, 0xB);
            assert_eq!(byte, 0x22);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }
    }

    #[test]
    fn try_into_test_4xnn() {
        let opcode = 0x4C37;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::CondVxNNNeq(reg, byte) = instr {
            assert_eq!(reg, 0xC);
            assert_eq!(byte, 0x37);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }
    }

    #[test]
    fn try_into_test_5xy0() {
        let opcode = 0x5CA0;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::CondVxVyEq(x, y) = instr {
            assert_eq!(x, 0xC);
            assert_eq!(y, 0xA);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }
    }

    #[test]
    fn try_into_test_6xnn() {
        let opcode = 0x65D7;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::ConstVxNN(reg, byte) = instr {
            assert_eq!(reg, 0x5);
            assert_eq!(byte, 0xD7);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }
    }

    #[test]
    fn try_into_test_7xnn() {
        let opcode = 0x7E15;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::ConstVxAddNN(reg, byte) = instr {
            assert_eq!(reg, 0xE);
            assert_eq!(byte, 0x15);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }
    }

    #[test]
    fn try_into_test_8xyn() {
        // 8xy0
        let opcode = 0x8420;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::AssignVxVy(x, y) = instr {
            assert_eq!(x, 0x4);
            assert_eq!(y, 0x2);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }

        // 8xy1
        let opcode = 0x8611;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::BitOpOR(x, y) = instr {
            assert_eq!(x, 0x6);
            assert_eq!(y, 0x1);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }

        // 8xy2
        let opcode = 0x8DA2;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::BitOpAND(x, y) = instr {
            assert_eq!(x, 0xD);
            assert_eq!(y, 0xA);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }

        // 8xy3
        let opcode = 0x8543;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::BitOpXOR(x, y) = instr {
            assert_eq!(x, 0x5);
            assert_eq!(y, 0x4);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }

        // 8xy4
        let opcode = 0x8AE4;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::MathVxVyAdd(x, y) = instr {
            assert_eq!(x, 0xA);
            assert_eq!(y, 0xE);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }

        // 8xy5
        let opcode = 0x8715;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::MathVxVySub(x, y) = instr {
            assert_eq!(x, 0x7);
            assert_eq!(y, 0x1);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }

        // 8xy6
        let opcode = 0x8166;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::BitOpShiftRight(x, y) = instr {
            assert_eq!(x, 0x1);
            assert_eq!(y, 0x6);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }

        // 8xy7
        let opcode = 0x8297;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::MathVyVxSub(x, y) = instr {
            assert_eq!(x, 0x2);
            assert_eq!(y, 0x9);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }

        // 8xyE
        let opcode = 0x893E;
        let instr = Instruction::try_from(opcode).unwrap();
        if let Instruction::BitOpShiftLeft(x, y) = instr {
            assert_eq!(x, 0x9);
            assert_eq!(y, 0x3);
        } else {
            panic!(
                "Opcode: {:?} failed to parse into correct instruction. Got {:?}",
                opcode, instr
            );
        }
    }
}
