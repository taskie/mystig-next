use glium_sdl2;

pub trait Game {
    fn update(&mut self) -> ();
    fn draw(&self, display: &mut glium_sdl2::Display) -> ();
    fn finished(&self) -> bool;
}
