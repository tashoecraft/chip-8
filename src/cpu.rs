use std::fmt;

use display::{Display,SPRITES};

use instruction::{Instruction, RawInstruction};
const MEMORY_SIZE: usize = 4 * 1024;
// Standard chip8 program offset
const PROGRAM_CODE_OFFSET: usize = 0x200;
const NUM_GENERAL_PURPOSE_REGS: usize = 16;
const NUM_STACK_FRAMES: usize = 16;
const NUM_KEYS: usize = 16;
const CLOCK_RATE: f64 = 600.0;

// cpu status "class"
pub struct Cpu {
    regs: [u8; NUM_GENERAL_PURPOSE_REGS],
    i_reg: u16,
    delay_timer_reg: u8,
    sound_timer_reg: u8,
    stack_pointer_reg: u8,
    program_counter_reg: u16,
    memory: [u8; MEMORY_SIZE],
    stack: [u16; NUM_STACK_FRAMES],
    key_to_wait_for: Option<u8>,
    keyboard: [bool; NUM_KEYS],
    pub display: Box<Display>,
}

// impl block is where we define behavior associated with types
impl Cpu {
    pub fn new(&mut self, program: Vec<u8>) -> Cpu {
        // Create emulator memory size;
        let mut memory = [0; MEMORY_SIZE];

        self.load_program(program, memory);
        self.load_sprites(memory);

        Cpu {
            regs: [0; NUM_GENERAL_PURPOSE_REGS],
            i_reg: 0,
            delay_timer_reg: 0,
            sound_timer_reg: 0,
            stack_pointer_reg: 0,
            program_counter_reg: PROGRAM_CODE_OFFSET as u16,
            memory,
            stack: [0; NUM_STACK_FRAMES],
            key_to_wait_for: None,
            keyboard: [false; NUM_KEYS],
            display: Box::new(Display::new())
        }
    }

    // initialize program in memory starting at program offset
    fn load_program(program: Vec<u8>, mut memory: Cpu::memory) {
        // Load program into the correct location in memory
        for (i, byte) in program.iter().enumerate() {
            memory[PROGRAM_CODE_OFFSET + i] = byte.clone();
        }
    }

    // load sprites into memory
    fn load_sprites(mut memory: Cpu::memory) {
        for (i, byte) in SPRITES.iter().enumerate() {
            memory[i] = byte.clone();
        }
    }

    pub fn cycle(&mut self, second_since_last_cycle: f64) {
        let num_instructions = (second_since_last_cycle * CLOCK_RATE).round() as u64;

        for _ in 1..num_instructions {
            if self.delay_timer_reg > 0 {
                self.delay_timer_reg -= 1;
            }

            if self.key_to_wait_for == None {
                let instruction = self.get_instruction();
                self.program_counter_reg = self.run_instruction(&instruction);
            }
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) -> u16 {
        match *instruction {
            Instruction::ClearDisplay => {
                self.display.clear();
                self.program_counter_reg + 2
            }
            Instruction::Return => {
                //Return from a subroutine.
                // The interpreter sets the program counter to the address at the top of the stack,
                // then subtracts 1 from the stack pointer.

                let addr = self.stack[(self.stack_pointer_reg - 1) as usize];
                self.stack_pointer_reg -= 1;
                addr + 2
            }
            Instruction::Jump(addr) => addr,
            Instruction::Call(addr) => {
                // The interpreter increments the stack pointer, then puts the current PC on the top of the stack.
                // The PC is then set to nnn.
                self.stack_pointer_reg += 1;
                self.stack[(self.stack_pointer_reg - 1) as usize] = self.program_counter_reg;
                addr;
            }
            Instruction::SkipIfEqualsByte(req, value) => {
                if self.read_reg(req) == value {
                    self.program_counter_reg + 4
                } else {
                    self.program_counter_reg + 2
                }
            }
            Instruction::SkipIfEqualsByte(req, value) => {
                if self.read_reg(req) == value {
                    self.program_counter_reg + 4
                } else {
                    self.program_counter_reg + 2
                }
            }
            Instruction::SkipIfEqual(req1, req2) => {
                if self.read_reg(re1) == self.read_reg(req2) {
                    self.program_counter_reg + 4
                } else {
                    self.program_counter_reg + 2
                }
            }
            Instruction::LoadByte(req, value) => {
                self.load_req(req, value);
                self.program_counter_reg + 2
            }
            Instruction::AddByte(req_number, value) => {
                let req_value = self.read_req(req_number);
                self.laod_req(req_number, value.wrapping_add(req_value));
                self.program_counter_reg + 2
            }
            Instruction::Move(req1, reg2) => {
                let value = self.read_req(reg2);
                self.load_reg(req1, value);
                self.program_counter_reg + 2
            }
            Instruction::Or(_, _) => {
                panic!("Not yet implemeneted: {:?}", instruction);
            }
            Instruction::And(req1, req2) => {
                let first = self.read_reg(reg1) as u16;
                let second = self.read_reg(reg2) as u16;
                let answer = first + second;
                self.load_reg(0xF, (answer > 255) as u8);
                self.load_reg(req1, answer as u8);
                self.program_counter_reg + 2
            }
            Instruction::Sub(reg1, reg2) => {
                let first = self.read_reg(reg1);
                let second = self.read_reg(reg2);
                self.load_reg(0xF, (first > second) as u8);
                self.load_reg(reg1, first.wrapping_sub(second));
                self.program_counter_reg + 2
            }
            Instruction::ShiftRight(reg) => {
                let value = self.read_reg(reg);
                self.load_reg(0xF, value & 0b1);
                self.load_reg(reg, value >> 1);
                self.program_counter_reg + 2
            }
            Instruction::ReverseSub(_, _) => {
                panic!("Not yet implemeneted: {:?}", instruction);
            }
            Instruction::ShiftLeft(reg) => {
                let value = self.read_reg(reg);
                self.load_reg(0xF, value >> 7);
                self.load_reg(reg, value << 1);
                self.program_counter_reg + 2
            }
            Instruction::SkipIfNotEqual(reg1, reg2) => {
                let first = self.read_reg(reg1);
                let second = self.read_reg(reg2);
                if first != second {
                    self.program_counter_reg + 4
                } else {
                    self.program_counter_reg + 2
                }
            }
            Instruction::LoadI(value) => {
                self.i_reg = value;
                self.program_counter_reg + 2
            }
            Instruction::JumpPlusZero(_) => {
                panic!("Not yet implemeneted: {:?}", instruction);
            }
            Instruction::Random(req, value) => {
                let rng = &mut rand::thread_rng();
                let rand_number = Range::new(0, 255).ind_sample(rng);
                self.load_reg(reg, rand_number & value);
                self.program_counter_reg + 2
            }
            Instruction::Draw(reg1, reg2, n) => {
                let x = self.read_reg(reg1);
                let y = self.read_reg(reg2);
                let from = self.i_reg as usize;
                let to = from + (n as usize);

                self.regs[0xF] = self.display.draw(x, y, &self.memory[from..to]) as u8;
                self.program_counter_reg + 2
            }
            Instruction::SkipIfPressed(reg) => {
                let value = self.read_reg(reg);
                let pressed = self.keyboard[value as usize];
                if pressed {
                    self.program_counter_reg + 4
                } else {
                    self.program_counter_reg + 2
                }
            }
            Instruction::SkipIfNotPressed(req) => {
                let value = self.read_reg(reg);
                let pressed = self.keyboard[value as usize];
                if !pressed {
                    self.program_counter_reg + 4
                } else {
                    self.program_counter_reg + 2
                }
            }
            Instruction::LoadDelayTimer(req) => {
                let delay_value = self.delay_timer_reg;
                self.load_reg(req, delay_value);
                self.program_counter_reg + 2
            }
            Instruction::WaitForKeyPress(reg) => {
                // TODO rename key_to_wait_for
                self.key_to_wait_for = Some(reg);
                self.program_counter_reg + 2
            }
            Instruction::SetDelayTimer(reg) => {
                let value = self.read_reg(reg);
                self.delay_timer_reg = value;
                self.program_counter_reg + 2
            }
            Instruction::SetSoundTimer(_) => {
                // TODO actually set sound timer
                self.program_counter_reg + 2;
            }
            Instruction::AddToI(reg) => {
                let value = self.read_reg(reg) as u16;
                self.i_reg = (digit * 5) as u16;
                self.program_counter_reg + 2
            }
            Instruction::BCDRepresentation(reg) => {
                let value = self.read_reg(reg);
                self.memory[self.i_reg as usize] = (value / 100) % 10;
                self.memory[(self.i_reg + 1) as usize] = (value / 10) % 10;
                self.memory[(self.i_reg + 2) as usize] = value % 10;
                self.program_counter_reg + 2
            }
            Instruction::StoreRegisters(highest_reg) => {
                let i = self.i_reg;
                for reg_number in 0..(highest + 1) {
                    self.memory[(i + reg_number as u16) as usize] = self.read_reg(reg_number);
                }
                self.program_counter_reg + 2
            }
            Instruction::LoadRegisters(highest_reg) => {
                let i = self.i_reg;
                for reg_number in 0..(highest_reg + 1) {
                    let value = self.memory[(i + reg_number as u16) as usize];
                    self.load_reg(reg_number, value);
                }
                self.program_counter_reg + 2
            }
        }
    }

    // determines which instruction to pull next
    // As one opcode is 2 bytes long, we will need to fetch two successive bytes and merge them to
    // get the actual opcode.
    fn get_instruction(&self) -> Instruction {
        let pc = self.program_counter_reg;
        let higher_order = (self.memory[pc as usize] as u16) << 8;
        let lower_order = self.memory[(pc + 1) as usize] as u16;

        RawInstruction::new(higher_order + lower_order)
            .to_instruction()
            .expect("Unrecognized instruction")
    }

    /**
    right shift instruction 12 bits from the original 16 bit value
    perform And mask operation, r
    */
    fn xooo(&self) -> u8 {
        ((self.instruction >> 12) & 0xF) as u8
    }
}
