use glium_sdl2;
use sdl2;

pub trait Game {
    fn process_event(&mut self, event: sdl2::event::Event);
    fn update(&mut self);
    fn draw(&self, display: &mut glium_sdl2::Display);
    fn finished(&self) -> bool;
}
