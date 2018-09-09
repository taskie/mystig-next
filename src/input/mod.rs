pub mod axis;
pub mod button;

pub mod joystick;
pub mod keyboard;
pub mod mouse;

pub use self::axis::Axis;
pub use self::button::Button;

#[derive(Debug)]
pub enum Device {
    Keyboard(keyboard::Keyboard),
    Joystick(joystick::Joystick),
    Mouse(mouse::Mouse),
}
