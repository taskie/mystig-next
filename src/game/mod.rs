pub trait Game {
    fn update(&mut self) -> ();
    fn draw(&self) -> ();
    fn finished(&self) -> bool;
}
