#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
extern crate simplelog;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::thread;
use std::time::Duration;

use simplelog::*;

pub mod gfx;
use gfx::spritesheet::colors::*;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 800;
const FPS: u32 = 60;

fn main() {

    CombinedLogger::init(
        vec![TermLogger::new(LevelFilter::Info, Config::default()).unwrap()]
    ).unwrap();

    info!("Starting Mallorn...");

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("M A L L O R N", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(GREY);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
