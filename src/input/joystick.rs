use super::button::Button;
use super::axis::Axes2D;

#[derive(Debug)]
pub struct Joystick {
    buttons: Vec<Button>,
    axes_vec: Vec<Axes2D>,
}

impl Joystick {
    pub fn new(buttons_len: usize, axes_vec_len: usize) -> Joystick {
        Joystick {
            buttons: vec![Button::new(); buttons_len],
            axes_vec: vec![Axes2D::new(0); axes_vec_len],
        }
    }

    pub fn update(&mut self) {
        for button in self.buttons.iter_mut() {
            button.update();
        }
        for axes in self.axes_vec.iter_mut() {
            axes.update();
        }
    }
}
