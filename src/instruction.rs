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
    pub fn new(instruction: u16) -> RawInstruction { RawInstruction { instruction}}

    pub fn to_instruction(&self) -> Option<Instruction> {
        match self.xooo() {
            0x0 => {
                // There are two 0x__ values
                match self.ooxx() {
                    // 11100000
                    0xE0 => Some(Instruction::ClearDisplay),
                    // 11101110
                    0xEE => Some(Instruction::Return),
                    _ => None,
                }
            }
            // 1-7 only contain 1 match, no need to check any other bits
            // Jump to location nnn.
            0x1 => Some(Instruction::Jump(self.oxxx())),
            // Call subroutine at nnn.
            0x2 => Some(Instruction::Call(self.oxxx())),
            // Skip next instruction if Vx = kk.
            0x3 => Some(Instruction::SkipIfEqualsByte(self.oxoo(), self.ooxx())),
            // Skip next instruction if Vx != kk.
            0x4 => Some(Instruction::SkipIfNotEqual(self.oxoo(), self.ooxx())),
            // Skip next instruction if Vx = Vy.
            0x5 => Some(Instruction::SkipIfEqual(self.oxoo(), self.ooxx())),
            // Set Vx = kk.
            0x6 => Some(Instruction::LoadByte(self.oxoo(), self.ooxx())),
            // Set Vx = Vx + kk.
            0x7 => Some(Instruction::AddByte(self.oxoo(), self.ooxx())),
            0x8 => {
                // 8 sub task to match on, all bit operators
                match self.ooox() {
                    0x0 => Some(Instruction::Move(self.oxoo(), self.ooxo())),
                    0x1 => Some(Instruction::Or(self.oxoo(), self.ooxo())),
                    0x2 => Some(Instruction::And(self.oxoo(), self.ooxo())),
                    0x3 => Some(Instruction::Xor(self.oxoo(), self.ooxo())),
                    0x4 => Some(Instruction::Add(self.oxoo(), self.ooxo())),
                    0x5 => Some(Instruction::Sub(self.oxoo(), self.ooxo())),
                    0x6 => Some(Instruction::ShiftRight(self.oxoo())),
                    0x7 => Some(Instruction::ReverseSub(self.oxoo(), self.ooxo())),
                    0xE => Some(Instruction::ShiftLeft(self.oxoo())),
                    _ => None,
                }
            }
            0x9 => Some(Instruction::SkipIfNotEqual(self.oxoo(), self.ooxo())),
            0xA => Some(Instruction::LoadI(self.oxxx())),
            0xB => Some(Instruction::JumpPlusZero(self.oxxx())),
            0xC => Some(Instruction::Random(self.oxoo(), self.ooxx())),
            0xD => Some(Instruction::Draw(self.oxoo(), self.ooxo(), self.ooox())),
            0xE => {
                match self.ooxx() {
                    0x9E => Some(Instruction::SkipIfPressed(self.oxoo())),
                    0xA1 => Some(Instruction::SkipIfNotPressed(self.oxoo())),
                    _ => None,
                }
            }
            0xF => {
                match self.ooxx() {
                    0x07 => Some(Instruction::LoadDelayTimer(self.oxoo())),
                    0x0A => Some(Instruction::WaitForKeyPress(self.oxoo())),
                    0x15 => Some(Instruction::SetDelayTimer(self.oxoo())),
                    0x18 => Some(Instruction::SetSoundTimer(self.oxoo())),
                    0x1E => Some(Instruction::AddToI(self.oxoo())),
                    0x29 => Some(Instruction::LoadSprite(self.oxoo())),
                    0x33 => Some(Instruction::BCDRepresentation(self.oxoo())),
                    0x55 => Some(Instruction::StoreRegisters(self.oxoo())),
                    0x65 => Some(Instruction::LoadRegisters(self.oxoo())),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    // Takes in a reference to self, grabs the value and bitwise shifts 12 to the right
    // Essentially just grabbing the left most bits. 1111 2222 3333 4444 -> 0000 0000 0000 1111
    // the & 0xF which is 1111, will convert the once 16 bit value to an 8bit value
    // and the & is a copy, so 1010 & 1111 -> will remain 1010. the end result being 0000 xxxx, as we
    // are zeroing out the left most nibble, and the right nibble is what were are planning on
    // matching against
    fn xooo(&self) -> u8 {
        ((self.value >> 12) & 0xF) as u8
    }

    fn oxoo(&self) -> u8 {
        ((self.value >> 8) & 0xF) as u8
    }

    fn ooxo(&self) -> u8 {
        ((self.value >> 4) & 0xF) as u8
    }

    fn ooox(&self) -> u8 {
        (self.value as u8) & 0xF
    }

    fn ooxx(&self) -> u8 {
        (self.value & 0xFF) as u8
    }

    // Returns a 16 bit value, 0000 xxxx xxxx xxxx, but we already know the left most nibble is 0000,
    // from the previous match
    fn oxxx(&self) -> u16 {
        self.value & 0xFFF
    }
}
