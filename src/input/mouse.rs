use super::button::Button;

#[derive(Clone, Debug)]
pub struct Mouse {
    pub x: i32,
    pub y: i32,
    pub button: Button,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            x: 0,
            y: 0,
            button: Button::new(),
        }
    }

    pub fn update(&mut self) {
        self.button.update();
    }
}
