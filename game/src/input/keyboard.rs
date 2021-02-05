use crate::events::{ElementState, VirtualKeyCode};
use std::collections::HashMap;
use winit::event::{KeyboardInput, ModifiersState};

pub struct KeyboardState {
    pressed_keys: HashMap<VirtualKeyCode, (ElementState, ModifiersState)>,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashMap::with_capacity(265),
        }
    }

    pub fn handle_keyboard_input(&mut self, input: KeyboardInput) {
        if let Some(virtual_keycode) = input.virtual_keycode {
            self.pressed_keys
                .insert(virtual_keycode, (input.state, input.modifiers));
        }
    }

    pub fn get_key_pressed(&self, key: &VirtualKeyCode) -> bool {
        match self.pressed_keys.get(key) {
            Some((state, _)) => match state {
                ElementState::Pressed => true,
                ElementState::Released => false,
            },
            None => false,
        }
    }
}
