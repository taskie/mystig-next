extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

mod input;

struct Application {
    sdl: Option<sdl2::Sdl>,
    canvas: Option<sdl2::render::Canvas<sdl2::video::Window>>,
    finished: bool,
}

impl Application {
    fn new() -> Application {
        Application {
            sdl: None,
            canvas: None,
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

    }
}

pub fn main() {
    let mut app = Application::new();
    app.start()

}
