//extern crate sdl2;
//extern crate rand;
//use sdl2::render::Canvas;
//use sdl2::video::Window;
//use sdl2::pixels::Color;
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
//use std::time::Duration;
//use rand::Rng;

mod cpu;

// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
// 0x200-0xFFF - Program ROM and work RAM
#[allow(unused_variables)]
fn main() {
   // new isntance of cpu "associated function"
   let cpu = &mut cpu::Cpu::new();
   cpu.load_program(vec![0x13, 0xC5]);
   // reset timers
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
//fn setup_graphics(screen: &mut [u8]) -> ! {
//    let sdl_context = sdl2::init().unwrap();
//    let video_subsystem = sdl_context.video().unwrap();
//
//    let window = video_subsystem.window("rust demo", 640, 320)
//        .position_centered()
//        .build()
//        .unwrap();
//
//    let mut canvas: Canvas<Window> = window
//        .into_canvas()
//        .present_vsync()
//        .build().unwrap();
//
//    canvas.set_scale(10.0, 10.0).unwrap();
//
//    canvas.clear();
//    canvas.present();
//    let mut event_pump = sdl_context.event_pump().unwrap();
//    let mut rng = rand::thread_rng();
//
//    let mut i = 0;
//    'running: loop {
//        i = (i + 1) % 255;
//        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
//
//        for (i, pixel) in screen.iter().enumerate() {
//            let val: u8 = rng.gen_range(0, 1);
//            screen[i] = val;
//        }
//
//        canvas.present();
//        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
//    }
//
//}
