use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

use super::button::Button;

#[derive(Debug)]
pub struct Keyboard {
    buttons: Vec<Button>,
}

impl Keyboard {
    pub fn new(len: usize) -> Keyboard {
        Keyboard { buttons: vec![Button::new(); len] }
    }

    pub fn update(&mut self) {
        for button in self.buttons.iter_mut() {
            button.update();
        }
    }
}
