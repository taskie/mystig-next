use super::game::Game;

pub struct Mystig {

}

impl Mystig {
    pub fn new() -> Mystig {
        Mystig { }
    }
}

impl Game for Mystig {
    fn update(&mut self) -> () {

    }

    fn draw(&self) -> () {

    }

    fn finished(&self) -> bool {
        false
    }
}