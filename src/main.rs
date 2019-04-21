extern crate sdl2;
extern crate rand;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::Rng;

// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
// 0x200-0xFFF - Program ROM and work RAM
#[allow(unused_variables)]
fn main() {
    let opcode: u16 = 0;
    let memory: [char; 4096];
    let v_registers: [char; 16];
    let index_regiser: u16;
    let program_counter: u16;

    let mut screen: [u8; 64 * 32] = [0; 64*32];



    let delay_timer: u16;
    let sound_timer: u16;

    let stack: [u16; 16];
    let stack_pointer: u16; 

    // HEX based keypad (0x0-0xF)
    let key: [char; 16];


    //setupGraphics
    //setupInput

    //initialize game
    // loadGame requested

    //Emulation loop
    setup_graphics(&screen)
    // loop {
        // emulate cycle

        // if chip8 vF flag, update graphics

        // store key press and release
    // }
}

fn setup_graphics(screen: &mut [u8]) -> ! {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust demo", 640, 320)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas: Canvas<Window> = window
        .into_canvas()
        .present_vsync()
        .build().unwrap();

    canvas.set_scale(10.0, 10.0).unwrap();

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = rand::thread_rng();

    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));

        for (i, pixel) in screen.iter().enumerate() {
            let val: u8 = rng.gen_range(0, 1);
            screen[i] = val;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}