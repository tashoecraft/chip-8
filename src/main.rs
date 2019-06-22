extern crate piston_window;
extern crate rand;

mod display;
mod instruction;
mod cpu;

use std::env;
use std::fs::File;
use std::io::Read;

use piston_window::*;

const ENLARGEMENT_FACTOR: usize = 20;
const WINDOW_DIMENSIONS: [u32; 2] = [(display::WIDTH * ENLARGEMENT_FACTOR) as u32,
                                     (display::HEIGHT * ENLARGEMENT_FACTOR) as u32];

// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
// 0x200-0xFFF - Program ROM and work RAM
#[allow(unused_variables)]
fn main() {
    let file_name = env::args().nth(1).expect("Must give game name as first file");
    let mut file = File::open(file_name).expect("There was an issue opening the file");
    let mut game_data = Vec::new();

    file.read_to_end(&mut game_data).expect("Failure to read file");

    let window: PistonWindow = WindowSettings::new("Chip-8 Emulator", WINDOW_DIMENSIONS)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut computer = cpu::Cpu::new(game_data);

    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            draw_screen(&computer.display.get_bugger(), &e);
        }

        if let Some(u) = e.update_args() {
            computer.cycle(u.dt);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            if let Some(key_value) = key_value(&key) {
                computer.handle_key_press(key_value);
            }
        }
    }
}

fn key_value(key: &Key) -> Option<u8> {
    if key.code() >= 48 && key.code() <= 57 {
        Some((key.code() - 48) as u8)
    } else if key.code() >= 97 && key.code() <= 102 {
        Some((key.code() - 97 + 10) as u8)
    } else {
        None
    }
}

fn draw_screen(display_buffer: &display::Buffer, window: &PistonWindow) {
    window.draw_2d(|context, graphics | {
        piston_window::clear(color::BLACK, graphics);

        for (i, row) in display_buffer.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                if *val {
                    let dimensions = [(j * ENLARGEMENT_FACTOR) as f64,
                        (i * ENLARGEMENT_FACTOR) as f64,
                        ENLARGEMENT_FACTOR as f64,
                        ENLARGEMENT_FACTOR as f64];
                    Rectangle::new(color::WHITE)
                        .draw(dimensions, &context.draw_state, context.transform, graphics);
                }
            }
        }
    })
}

// Notes
// working with hex and bytes flow well together, two hex digits takes 8 bytes to rep
// each byte of memory cna hold values 0x0 -> 0xFF, each instruction and opcode is two bytes long
// and memory addresses range form 0x0 -> 0xFFFF (only -> 0xFF are valid in our system)

// general emulator process
// 1 Look at instruction in memory at the location of our program counter
// 2 Figure out what that instruction means to do (decode)
// 3 perform that instruction
// 4 Update the program counter
// 5 Repeat indefinitely
//
//
