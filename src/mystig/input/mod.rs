use super::super::input;
use std::collections::HashMap;

pub trait Resettable {
    fn reset(&mut self);
}

impl Resettable for input::Button {
    fn reset(&mut self) {
        input::Button::reset(self)
    }
}

pub trait Updatable {
    fn update(&mut self);
}

impl Updatable for input::Button {
    fn update(&mut self) {
        input::Button::update(self)
    }
}

pub struct Input<T: Default + Resettable + Updatable> {
    map: HashMap<String, usize>,
    states: Vec<T>,
}

impl<T: Default + Resettable + Updatable> Input<T> {
    pub fn new(size: usize) -> Input<T> {
        let mut input = Input {
            map: HashMap::new(),
            states: Vec::with_capacity(size),
        };
        for i in 0..size {
            input.states.push(T::default())
        }
        input
    }

    pub fn reset(&mut self) {
        for state in self.states.iter_mut() {
            state.reset()
        }
    }

    pub fn update(&mut self) {
        for state in self.states.iter_mut() {
            state.update()
        }
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.map.get(key).and_then(|index| self.states.get(*index))
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        let states = &mut self.states;
        self.map.get(key).and_then(move |index| states.get_mut(*index))
    }

    pub fn register(&mut self, key: &str, index: usize) {
        self.map.insert(String::from(key), index);
    }
}

pub fn get_button<'s>(state: &'s mut State, mapper: &Mapper, key: &str) -> Option<&'s input::Button> {
    if let Some(index) = mapper.button_map.get(key) {
        state.buttons.get(*index)
    } else {
        None
    }
}

pub fn get_button_mut<'s>(state: &'s mut State, mapper: &Mapper, key: &str) -> Option<&'s mut input::Button> {
    if let Some(index) = mapper.button_map.get(key) {
        state.buttons.get_mut(*index)
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Mapper {
    pub button_map: HashMap<String, usize>,
    pub axis_map: HashMap<String, usize>,
}

impl Mapper {
    pub fn new() -> Mapper {
        Mapper {
            button_map: HashMap::new(),
            axis_map: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub buttons: Vec<input::button::Button>,
    pub axes: Vec<input::axis::Axis>,
}

impl State {
    pub fn new() -> State {
        State {
            buttons: vec![],
            axes: vec![],
        }
    }

    pub fn update(&mut self) {
        for button in self.buttons.iter_mut() {
            button.update()
        }
        for axis in self.axes.iter_mut() {
            axis.update()
        }
    }

    pub fn reset(&mut self) {
        for button in self.buttons.iter_mut() {
            button.reset()
        }
        for axis in self.axes.iter_mut() {
            axis.reset()
        }
    }
}
