use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use super::button::Button;

#[derive(Debug)]
pub struct Keyboard {
    pub buttons: Vec<Button>,
}

impl Keyboard {
    pub fn new(len: usize) -> Keyboard {
        Keyboard {
            buttons: vec![Button::new(); len],
        }
    }

    pub fn update(&mut self) {
        for button in self.buttons.iter_mut() {
            button.update();
        }
    }
}
