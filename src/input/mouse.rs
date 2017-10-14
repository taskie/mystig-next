use super::button::Button;

#[derive(Clone, Debug)]
pub struct Mouse {
    x: i32,
    y: i32,
    button: Button,
}

impl Mouse {
    fn new() -> Mouse {
        Mouse {
            x: 0,
            y: 0,
            button: Button::new(),
        }
    }

    fn update(&mut self) {
        self.button.update();
    }
}
