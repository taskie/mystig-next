use super::button::Button;
use super::axis::Axis2D;

#[derive(Debug)]
pub struct Joystick {
    buttons: Vec<Button>,
    axes: Vec<Axis2D>,
}

impl Joystick {
    pub fn new(buttons_len: usize, axes_len: usize) -> Joystick {
        Joystick {
            buttons: vec![Button::new(); buttons_len],
            axes: vec![Axis2D::new(); axes_len],
        }
    }

    pub fn update(&mut self) {
        for button in self.buttons.iter_mut() {
            button.update();
        }
        for axis in self.axes.iter_mut() {
            axis.update();
        }
    }
}
