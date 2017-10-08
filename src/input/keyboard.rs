use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

use super::button::Button;

#[derive(Debug)]
struct Keyboard<Keycode: Clone + Debug + Eq + Hash> {
    button_id_for_keycode: HashMap<Keycode, usize>,
    button_for_button_id: Vec<Button>,
}

impl <Keycode> Keyboard<Keycode> where Keycode: Clone + Debug + Eq + Hash {
    pub fn new(keycodes: Vec<Keycode>) -> Keyboard<Keycode> {
        let button_for_button_id = vec![Button::new(); keycodes.len()];
        let button_id_for_keycode =
            keycodes
                .iter()
                .enumerate()
                .map(|(i, kc)| (kc.clone(), i))
                .collect();
        Keyboard {
            button_id_for_keycode,
            button_for_button_id,
        }
    }

    pub fn up(&mut self, keycode: Keycode) {
        self.resolve_button_mut(keycode).map(|button| button.up());
    }

    pub fn resolve_button(&self, keycode: Keycode) -> Option<&Button> {
        self.button_id_for_keycode
            .get(&keycode)
            .map(|&button_id| self.button_for_button_id.get(button_id).unwrap())
    }

    pub fn resolve_button_mut(&mut self, keycode: Keycode) -> Option<&mut Button> {
        self.button_id_for_keycode
            .get(&keycode)
            .map(|&button_id| {
                let button_id = button_id;
                self.button_for_button_id.get_mut(button_id).unwrap()
            })
    }
}

#[test]
fn test_1 () {
    let mut keyboard = Keyboard::new(vec![3, 4, 7, 9]);
    keyboard.up(3);
    keyboard.up(7);
    println!("{:?}", keyboard);
    assert!(false)
}