pub mod button;
pub mod axis;

pub mod keyboard;
pub mod joystick;
pub mod mouse;

#[derive(Debug)]
pub enum Device {
    Keyboard(keyboard::Keyboard),
    Joystick(joystick::Joystick),
    Mouse(mouse::Mouse),
}
