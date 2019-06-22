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
        10
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
