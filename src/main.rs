extern crate sdl2;
extern crate rlua;
extern crate rmp;
extern crate enum_map;
#[macro_use]
extern crate enum_map_derive;
extern crate glium;
mod glium_sdl2;

use std::time::{Duration, Instant};

use sdl2::video;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rlua::{Lua, MultiValue, Error};
use glium::Surface;
use glium_sdl2::DisplayBuild;

mod input;
mod game;
mod mystig;

struct Application<GameT: game::Game> {
    sdl: Option<sdl2::Sdl>,
    display: Option<glium_sdl2::Display>,
    game: GameT,
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

        let display = video_subsystem
            .window("rust-sdl2 demo: Video", 640, 480)
            .position_centered()
            .opengl()
            .build_glium()
            .unwrap();

        self.sdl = Some(sdl);
        self.display = Some(display);
        self
    }

    fn run(&mut self) {
        let sdl = self.sdl.clone().unwrap();
        let mut event_pump = sdl.event_pump().unwrap();

        'running: loop {
            let now = Instant::now();

            for event in event_pump.poll_iter() {
                self.process_event(event);
            }
            self.update();

            if self.finished {
                break 'running;
            }

            self.draw();

            let elapsed = now.elapsed();
            let asleep_time = Duration::new(0, 1_000_000_000 / 60);
            if let Some(nanos) = asleep_time.checked_sub(elapsed) {
                ::std::thread::sleep(nanos);
            } else {
                ::std::thread::sleep(Duration::new(0, 0));
            }
            self.frame += 1;
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
        self.game.draw();
        let s: String = self.frame.to_string();
        let mut target = self.display.clone().unwrap().draw();
        target.clear_color_and_depth((0.5, 0.5, 1.0, 0.0), 1.0);
        target.finish().unwrap();
    }
}

pub fn main() {
    let mut app = Application::new(mystig::Mystig::new());
    app.start()
}
