extern crate sdl2;
extern crate hlua;
extern crate rmp;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

mod input;
mod game;
mod mystig;

struct Application<GameT: game::Game> {
    sdl: Option<sdl2::Sdl>,
    canvas: Option<sdl2::render::Canvas<sdl2::video::Window>>,
    game: GameT,
    finished: bool,
}

impl <GameT> Application<GameT> where GameT: game::Game {
    fn new(game: GameT) -> Application<GameT> {
        Application::<GameT> {
            sdl: None,
            canvas: None,
            game,
            finished: false,
        }
    }

    fn start(&mut self) {
        self.prepare().run()
    }

    fn prepare(&mut self) -> &mut Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo: Video", 640, 480)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        self.sdl = Some(sdl);
        self.canvas = Some(canvas);
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
                break 'running
            }

            self.draw();

            let elapsed = now.elapsed();
            let asleep_time = Duration::new(0, 1_000_000_000 / 60);
            if let Some(nanos) = asleep_time.checked_sub(elapsed) {
                ::std::thread::sleep(nanos);
            } else {
                ::std::thread::sleep(Duration::new(0, 0));
            }
        }
    }


    fn process_event(&mut self, event: Event) {
        match event {
            Event::Quit {..} => {
                self.finished = true;
            },
            Event::KeyDown { keycode: Some(keycode), .. } => {

            },
            Event::KeyUp { keycode: Some(keycode), .. } => {

            },
            _ => {}
        }
    }

    fn update(&mut self) {
        self.game.update();
        if ! self.finished {
            self.finished = self.game.finished();
        }
    }

    fn draw(&mut self) {
        self.canvas.as_mut().unwrap().clear();
        self.game.draw();
        self.canvas.as_mut().unwrap().present();
    }
}

pub fn main() {
    let mut app = Application::new(mystig::Mystig::new());
    app.start()
}
