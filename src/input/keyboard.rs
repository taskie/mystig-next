use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

use super::button::Button;

#[derive(Debug)]
struct Keyboard<KeycodeT: Clone + Debug + Eq + Hash> {
    button_id_for_keycode: HashMap<KeycodeT, usize>,
    button_for_button_id: Vec<Button>,
}

impl <KeycodeT> Keyboard<KeycodeT> where KeycodeT: Clone + Debug + Eq + Hash {
    pub fn new(keycodes: Vec<KeycodeT>) -> Keyboard<KeycodeT> {
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

    pub fn up(&mut self, keycode: KeycodeT) {
        self.resolve_button_mut(keycode).map(|button| button.up());
    }

    pub fn down(&mut self, keycode: KeycodeT) {
        self.resolve_button_mut(keycode).map(|button| button.down());
    }

    pub fn update(&mut self) {
        self.button_for_button_id
            .iter_mut()
            .for_each(|button| button.update());
    }

    pub fn resolve_button(&self, keycode: KeycodeT) -> Option<&Button> {
        self.button_id_for_keycode
            .get(&keycode)
            .and_then(|&button_id| self.button_for_button_id.get(button_id))
    }

    pub fn resolve_button_mut(&mut self, keycode: KeycodeT) -> Option<&mut Button> {
        match self.button_id_for_keycode.get(&keycode) {
            Some(&button_id) => { self.button_for_button_id.get_mut(button_id) },
            _ => { None },
        }
    }
}

#[test]
fn test_1 () {
    let mut keyboard = Keyboard::new(vec![2, 3, 5, 7]);
    keyboard.down(3);
    keyboard.down(5);
    keyboard.down(7);
    keyboard.update();

    keyboard.up(3);
    keyboard.up(5);
    keyboard.update();

    keyboard.down(3);
    keyboard.update();

    println!("{:?}", keyboard);
    assert!(vec![3].iter()
        .all(|&k| keyboard.resolve_button(k).unwrap().just()));
    assert!(vec![3, 7].iter()
        .all(|&k| keyboard.resolve_button(k).unwrap().pressed()));
    assert!(vec![2, 5].iter()
        .all(|&k| ! keyboard.resolve_button(k).unwrap().pressed()));
}