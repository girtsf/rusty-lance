extern crate piston;

use super::buttons;
use piston::input;
use std::clone::Clone;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Key {
    Left,
    Right,
    Up,
}

struct KeyMappingTarget {
    player: usize,
    key: Key,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
lazy_static! {
    static ref KEY_MAPPING: HashMap<input::Key, KeyMappingTarget> = {
        let mut m = HashMap::new();
        m.insert(input::Key::Left, KeyMappingTarget{player:0, key:Key::Left});
        m.insert(input::Key::Right, KeyMappingTarget{player:0, key:Key::Right});
        m.insert(input::Key::Up, KeyMappingTarget{player:0, key:Key::Up});
        m.insert(input::Key::A, KeyMappingTarget{player:1, key:Key::Left});
        m.insert(input::Key::D, KeyMappingTarget{player:1, key:Key::Right});
        m.insert(input::Key::W, KeyMappingTarget{player:1, key:Key::Up});
        m
    };
}

/// Tracks what buttons each player is pressing.
pub struct ButtonTracker {
    held: [HashSet<Key>; 2],
}

impl ButtonTracker {
    pub fn new() -> Self {
        ButtonTracker {
            held: [HashSet::new(), HashSet::new()],
        }
    }

    pub fn handle(&mut self, button_args: &input::ButtonArgs) {
        // println!("key: {:?}", button_args);
        if let input::Button::Keyboard(input_key) = button_args.button {
            if let Some(kmt) = KEY_MAPPING.get(&input_key) {
                let player = kmt.player;
                let key = kmt.key.clone();
                match button_args.state {
                    input::ButtonState::Press => {
                        self.held[player].insert(key);
                    }
                    input::ButtonState::Release => {
                        self.held[player].remove(&key);
                    }
                };
            }
        }
    }

    pub fn is_key_held(&self, player: usize, key: buttons::Key) -> bool {
        self.held[player].contains(&key)
    }
}
