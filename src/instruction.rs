pub type Address = u16;
pub type Register = u8;

pub enum Instruction {
    ClearDisplay,
    Return,
    Jump(Address),
    Call(Address),
    SkipIfEqualsByte(Register, u8),
    SkipIfNotEqualsByte(Register, u8),
    SkipIfEqual(Register, Register),
    LoadByte(Register, u8),
    AddByte(Register, u8),
    Move(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    Xor(Register, Register),
    Add(Register, Register),
    Sub(Register, Register),
    ShiftRight(Register),
    ReverseSub(Register, Register),
    ShiftLeft(Register),
    SkipIfNotEqual(Register, Register),
    LoadI(u16),
    JumpPlusZero(Address),
    Random(Register, u8),
    Draw(Register, Register, u8),
    SkipIfPressed(Register),
    SkipIfNotPressed(Register),
    LoadDelayTimer(Register),
    WaitForKeyPress(Register),
    SetDelayTimer(Register),
    SetSoundTimer(Register),
    AddToI(Register),
    LoadSprite(Register),
    BCDRepresentation(Register),
    StoreRegisters(Register),
    LoadRegisters(Register)
}

pub struct RawInstruction {
    instruction: u16
}

impl RawInstruction {

    pub fn instruction(instruction: u16) -> Option<Instruction> {
        match xooo(instruction) {
            0x0 => {
                match ooxx(instruction) {
                    0xE0 => Some(Instruction::ClearDisplay),
                    0xEE => Some(Instruction::Return),
                    _ => None,
                }
            }
            0x1 => Some(Instruction::Jump(oxxx(instruction))),
            0x2 => Some(Instruction::Call(oxxx(instruction))),
            0x3 => Some(Instruction::SkipIfEqualsByte(oxoo(instruction), ooxx(instruction))),
            0x4 => Some(Instruction::SkipIfNotEqualsByte(oxoo(instruction), ooxx(instruction))),
            0x5 => Some(Instruction::SkipIfEqual(oxoo(instruction), ooxo(instruction))),
            0x6 => Some(Instruction::LoadByte(oxoo(instruction), ooxx(instruction))),
            0x7 => Some(Instruction::AddByte(oxoo(instruction), ooxx(instruction))),
            0x8 => {
                match ooox(instruction) {
                    0x0 => Some(Instruction::Move(oxoo(instruction), ooxo(instruction))),
                    0x1 => Some(Instruction::Or(oxoo(instruction), ooxo(instruction))),
                    0x2 => Some(Instruction::And(oxoo(instruction), ooxo(instruction))),
                    0x3 => Some(Instruction::Xor(oxoo(instruction), ooxo(instruction))),
                    0x4 => Some(Instruction::Add(oxoo(instruction), ooxo(instruction))),
                    0x5 => Some(Instruction::Sub(oxoo(instruction), ooxo(instruction))),
                    0x6 => Some(Instruction::ShiftRight(oxoo(instruction))),
                    0x7 => Some(Instruction::ReverseSub(oxoo(instruction), ooxo(instruction))),
                    0xE => Some(Instruction::ShiftLeft(oxoo(instruction))),
                    _ => None,
                }
            }
            0x9 => Some(Instruction::SkipIfNotEqual(oxoo(instruction), ooxo(instruction))),
            0xA => Some(Instruction::LoadI(oxxx(instruction))),
            0xB => Some(Instruction::JumpPlusZero(oxxx(instruction))),
            0xC => Some(Instruction::Random(oxoo(instruction), ooxx(instruction))),
            0xD => Some(Instruction::Draw(oxoo(instruction), ooxo(instruction), ooox(instruction))),
            0xE => {
                match ooxx(instruction) {
                    0x9E => Some(Instruction::SkipIfPressed(oxoo(instruction))),
                    0xA1 => Some(Instruction::SkipIfNotPressed(oxoo(instruction))),
                    _ => None,
                }
            }
            0xF => {
                match ooxx(instruction) {
                    0x07 => Some(Instruction::LoadDelayTimer(oxoo(instruction))),
                    0x0A => Some(Instruction::WaitForKeyPress(oxoo(instruction))),
                    0x15 => Some(Instruction::SetDelayTimer(oxoo(instruction))),
                    0x18 => Some(Instruction::SetSoundTimer(oxoo(instruction))),
                    0x1E => Some(Instruction::AddToI(oxoo(instruction))),
                    0x29 => Some(Instruction::LoadSprite(oxoo(instruction))),
                    0x33 => Some(Instruction::BCDRepresentation(oxoo(instruction))),
                    0x55 => Some(Instruction::StoreRegisters(oxoo(instruction))),
                    0x65 => Some(Instruction::LoadRegisters(oxoo(instruction))),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

// Takes in a reference to self, grabs the value and bitwise shifts 12 to the right
// Essentially just grabbing the left most bits. 1111 2222 3333 4444 -> 0000 0000 0000 1111
// the & 0xF which is 1111, will convert the once 16 bit value to an 8bit value
// and the & is a copy, so 1010 & 1111 -> will remain 1010. the end result being 0000 xxxx, as we
// are zeroing out the left most nibble, and the right nibble is what were are planning on
// matching against
fn xooo(instruction: u16) -> u8 {
    ((instruction >> 12) & 0xF) as u8
}

fn oxoo(instruction: u16) -> u8 {
    ((instruction >> 8) & 0xF) as u8
}

fn ooxo(instruction: u16) -> u8 {
    ((instruction >> 4) & 0xF) as u8
}

fn ooox(instruction: u16) -> u8 {
    (instruction as u8) & 0xF
}

fn ooxx(instruction: u16) -> u8 {
    (instruction & 0xFF) as u8
}

// Returns a 16 bit value, 0000 xxxx xxxx xxxx, but we already know the left most nibble is 0000,
// from the previous match
fn oxxx(instruction: u16) -> u16 {
    instruction & 0xFFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xooo() {
        let zero = 0b0000_1111_1111_1111;
        let mix = 0b1010_1111_1111_1111;
        let ones = 0b1111_1111_1111_1111;

        assert_eq!(xooo(zero), 0b0000_0000);
        assert_eq!(xooo(mix), 0b0000_1010);
        assert_eq!(xooo(ones), 0b0000_1111);
    }

    #[test]
    fn test_oxoo() {
        let zero = 0b0000_0000_1111_1111;
        let mix = 0b0000_1010_1111_1111;
        let ones = 0b1111_1111_1111_1111;

        assert_eq!(oxoo(zero), 0b0000_0000);
        assert_eq!(oxoo(mix), 0b0000_1010);
        assert_eq!(oxoo(ones), 0b0000_1111);
    }

    #[test]
    fn test_ooxo() {
        let zero = 0b0000_0000_0000_1111;
        let mix = 0b0000_1111_1010_1111;
        let ones = 0b1111_1111_1111_1111;

        assert_eq!(ooxo(zero), 0b0000_0000);
        assert_eq!(ooxo(mix), 0b0000_1010);
        assert_eq!(ooxo(ones), 0b0000_1111);
    }

    #[test]
    fn test_ooox() {
        let zero = 0b0000_0000_0000_0000;
        let mix = 0b0000_1111_1010_1010;
        let ones = 0b1111_1111_1111_1111;

        assert_eq!(ooox(zero), 0b0000_0000);
        assert_eq!(ooox(mix), 0b0000_1010);
        assert_eq!(ooox(ones), 0b0000_1111);
    }

    #[test]
    fn test_ooxx() {
        let zero = 0b0000_0000_0000_0000;
        let mix = 0b0000_1111_1010_1010;
        let ones = 0b1111_1111_1111_1111;

        assert_eq!(ooxx(zero), 0b0000_0000);
        assert_eq!(ooxx(mix), 0b1010_1010);
        assert_eq!(ooxx(ones), 0b1111_1111);
    }

    #[test]
    fn test_oxxx() {
        let zero = 0b0000_0000_0000_0000;
        let mix = 0b0000_1010_1010_1010;
        let ones = 0b1111_1111_1111_1111;

        assert_eq!(oxxx(zero), 0b000_0000_0000);
        assert_eq!(oxxx(mix), 0b1010_1010_1010);
        assert_eq!(oxxx(ones), 0b1111_1111_1111);
    }

}
