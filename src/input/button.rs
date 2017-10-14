#[derive(Clone, Debug, Default)]
pub struct Button {
    status: i32,
}

impl Button {
    pub fn new() -> Button {
        Button { status: -1 }
    }

    pub fn up(&mut self) {
        if self.status >= 0 {
            self.status = -1
        }
    }

    pub fn down(&mut self) {
        if self.status < 0 {
            self.status = 0
        }
    }

    pub fn update(&mut self) {
        if self.status >= 0 {
            self.status += 1;
        } else {
            self.status -= 1;
        }
    }

    pub fn pressed(&self) -> bool {
        self.status >= 0
    }

    pub fn just(&self) -> bool {
        self.holding_time() == 1
    }

    pub fn holding_time(&self) -> i32 {
        if self.pressed() {
            debug_assert!(self.status >= 0);
            self.status
        } else {
            debug_assert!(!self.status >= 0);
            !self.status
        }
    }
}

#[test]
fn test_1() {
    let mut b = Button::new();

    assert!(!b.pressed());
    assert!(!b.just());

    b.down();
    assert!(b.pressed());
    assert!(!b.just());
    b.update();
    assert!(b.just());
    b.update();
    assert_eq!(b.holding_time(), 2);
    assert!(!b.just());

    b.up();
    assert!(!b.pressed());
    assert!(!b.just());
    b.update();
    assert!(b.just());
    b.update();
    assert_eq!(b.holding_time(), 2);
    assert!(!b.just())
}
