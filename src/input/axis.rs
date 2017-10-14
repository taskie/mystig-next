use super::button::Button;

#[derive(Clone, Debug)]
pub struct Axis {
    value: i32,
    play: i32,
    negative_button: Button,
    positive_button: Button,
}

impl Axis {
    pub fn new(play: i32) -> Axis {
        Axis {
            value: 0,
            negative_button: Button::new(),
            positive_button: Button::new(),
            play,
        }
    }

    pub fn set(&mut self, value: i32) {
        self.value = value
    }

    pub fn get(&self) -> i32 {
        self.value
    }

    pub fn update(&mut self) {
        if self.value < -self.play {
            self.negative_button.down();
            self.positive_button.up();
        } else if self.value > self.play {
            self.negative_button.up();
            self.positive_button.down();
        } else {
            self.negative_button.up();
            self.positive_button.up();
        }
        self.negative_button.update();
        self.positive_button.update();
    }
}

#[derive(Clone, Debug)]
pub struct Axes2D {
    x: Axis,
    y: Axis,
}

impl Axes2D {
    pub fn new(play: i32) -> Axes2D {
        Axes2D {
            x: Axis::new(play),
            y: Axis::new(play),
        }
    }

    pub fn set(&mut self, x: i32, y: i32) {
        self.x.set(x);
        self.y.set(y);
    }

    pub fn get(&self) -> (i32, i32) {
        (self.x.get(), self.y.get())
    }

    pub fn update(&mut self) {
        self.x.update();
        self.y.update();
    }
}

#[test]
fn test_1() {
    let mut axis = Axes2D::new(0);

    axis.set(-1, 1);
    axis.update();
    assert!(axis.x.negative_button.pressed());
    assert!(axis.x.negative_button.just());
    assert!(! axis.x.positive_button.pressed());
    assert!(axis.x.positive_button.just());
    assert!(! axis.y.negative_button.pressed());
    assert!(axis.y.negative_button.just());
    assert!(axis.y.positive_button.pressed());
    assert!(axis.y.positive_button.just());

    axis.set(-2, -2);
    axis.update();
    assert!(axis.x.negative_button.pressed());
    assert!(! axis.x.negative_button.just());
    assert!(! axis.x.positive_button.pressed());
    assert!(! axis.x.positive_button.just());
    assert!(axis.y.negative_button.pressed());
    assert!(axis.y.negative_button.just());
    assert!(! axis.y.positive_button.pressed());
    assert!(axis.y.positive_button.just());

    axis.set(0, 0);
    axis.update();
    assert!(! axis.x.negative_button.pressed());
    assert!(axis.x.negative_button.just());
    assert!(! axis.x.positive_button.pressed());
    assert!(! axis.x.positive_button.just());
    assert!(!axis.y.negative_button.pressed());
    assert!(axis.y.negative_button.just());
    assert!(! axis.y.positive_button.pressed());
    assert!(! axis.y.positive_button.just());
}
