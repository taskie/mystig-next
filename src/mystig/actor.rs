pub trait Actor {
    fn update(&mut self);
    fn is_dead(&self) -> bool;
    fn draw(&self) {}
}
