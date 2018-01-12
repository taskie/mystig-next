extern crate sdl2;
extern crate rlua;
extern crate rmp;
extern crate nalgebra;
#[macro_use]
extern crate glium;
mod glium_sdl2;
#[cfg(target_os = "emscripten")]
pub mod emscripten_helper;

use std::time::{Duration, Instant};

use sdl2::video;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rlua::{Lua, MultiValue, Error};
use glium::Surface;
use glium_sdl2::DisplayBuild;
#[cfg(target_os = "emscripten")]
use emscripten_helper::{emscripten};

mod input;
mod game;
mod loader;
mod mystig;


struct Application<GameT: game::Game> {
    sdl: Option<sdl2::Sdl>,
    display: Option<glium_sdl2::Display>,
    game: GameT,
    loader: loader::Loader,
    finished: bool,
    frame: i32,
}

impl<GameT> Application<GameT>
where
    GameT: game::Game,
{
    fn new(game: GameT) -> Application<GameT> {
        Application::<GameT> {
            sdl: None,
            display: None,
            game,
            loader: loader::Loader::new(),
            finished: false,
            frame: 0,
        }
    }

    fn start(&mut self) {
        self.prepare().run()
    }

    fn prepare(&mut self) -> &mut Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();

        video_subsystem.gl_attr().set_depth_size(24);
        video_subsystem.gl_attr().set_multisample_buffers(1);
        video_subsystem.gl_attr().set_multisample_samples(4);
        video_subsystem.gl_attr().set_context_major_version(3);
        if cfg!(target_os = "macos") {
            video_subsystem.gl_attr().set_context_profile(
                sdl2::video::GLProfile::Core,
            );
        }

        let display = video_subsystem
            .window("Rust SDL Lua", 640, 480)
            .position_centered()
            .build_glium()
            .unwrap();

        self.sdl = Some(sdl);
        self.display = Some(display);
        self.load();
        self
    }

    fn load(&mut self) {}

    fn run(&mut self) {
        let sdl = self.sdl.clone().unwrap();
        let mut event_pump = sdl.event_pump().unwrap();

        let mut main_loop = || {
            let now = Instant::now();

            for event in event_pump.poll_iter() {
                self.process_event(event);
            }
            self.update();

            if self.finished {
                return false
            }

            self.draw();

            let elapsed = now.elapsed();
            let asleep_time = Duration::new(0, 1_000_000_000 / 60);
            if let Some(nanos) = asleep_time.checked_sub(elapsed) {
                std::thread::sleep(nanos);
            } else {
                std::thread::sleep(Duration::new(0, 0));
            }
            self.frame += 1;
            return true
        };

        #[cfg(target_os = "emscripten")]
        emscripten::set_main_loop_callback(|| { main_loop(); });

        #[cfg(not(target_os = "emscripten"))]
        loop {
            if ! main_loop() {
                break
            }
        }
    }


    fn process_event(&mut self, event: Event) {
        match event {
            Event::Quit { .. } => {
                self.finished = true;
            }
            Event::KeyDown { keycode: Some(keycode), .. } => {}
            Event::KeyUp { keycode: Some(keycode), .. } => {}
            Event::MouseButtonDown { x, y, .. } => {}
            _ => {}
        }
    }

    fn update(&mut self) {
        self.game.update();
        if !self.finished {
            self.finished = self.game.finished();
        }
    }

    fn draw(&mut self) {
        let mut display = self.display.clone().unwrap();
        self.game.draw(&mut display);
    }
}

pub fn main() {
    let mut app = Application::new(mystig::Mystig::new());
    app.start()
}
