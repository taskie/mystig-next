use super::button::Button;

use enum_map::EnumMap;

#[derive(Debug, EnumMap)]
pub enum ButtonId {
    XIsNegative,
    XIsPositive,
    YIsNegative,
    YIsPositive,
}

#[derive(Clone, Debug)]
pub struct Axis2D {
    x: i32,
    y: i32,
    buttons: EnumMap<ButtonId, Button>,
}

impl Axis2D {
    pub fn new() -> Axis2D {
        Axis2D {
            x: 0,
            y: 0,
            buttons: EnumMap::default(),
        }
    }

    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn update(&mut self) {
        if self.x < 0 {
            self.buttons[ButtonId::XIsNegative].down();
            self.buttons[ButtonId::XIsPositive].up();
        } else if self.x > 0 {
            self.buttons[ButtonId::XIsNegative].up();
            self.buttons[ButtonId::XIsPositive].down();
        } else {
            self.buttons[ButtonId::XIsNegative].up();
            self.buttons[ButtonId::XIsPositive].up();
        }

        if self.y < 0 {
            self.buttons[ButtonId::YIsNegative].down();
            self.buttons[ButtonId::YIsPositive].up();
        } else if self.y > 0 {
            self.buttons[ButtonId::YIsNegative].up();
            self.buttons[ButtonId::YIsPositive].down();
        } else {
            self.buttons[ButtonId::YIsNegative].up();
            self.buttons[ButtonId::YIsPositive].up();
        }

        for button in self.buttons.values_mut() {
            button.update()
        }
    }
}

#[test]
fn test_1() {
    let mut axis = Axis2D::new();
    axis.set(-1, 0);
    axis.update();
    assert!(axis.buttons[ButtonId::XIsNegative].pressed());
}
