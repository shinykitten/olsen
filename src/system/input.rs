use bevy::prelude::*;
use crate::system::{Facing, PlayerAction, PlayerStateEvent};
use std::collections::HashSet;
use lazy_static::*;

lazy_static! {
    static ref DIRECTIONAL_KEYS: HashSet<KeyCode> = {
        let mut s = HashSet::new();
        s.insert(KeyCode::W);
        s.insert(KeyCode::A);
        s.insert(KeyCode::S);
        s.insert(KeyCode::D);
        s
    };
}

#[derive(Default)]
pub struct State {
    pressed_keys: HashSet<KeyCode>,
}

// A basic input handler that just watches for WASD keys and fires off the corresponding PlayerStateEvents.
pub fn input_event_receiver(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<PlayerStateEvent>,
    mut state: Local<State>,
) {
    for key in keyboard_input.get_just_pressed() {
        state.pressed_keys.insert(*key);
        match key {
            KeyCode::W => { event_writer.send(PlayerStateEvent{ state: PlayerAction::Run, direction: Some(Facing::Up) })},
            KeyCode::A => { event_writer.send(PlayerStateEvent{ state: PlayerAction::Run, direction: Some(Facing::Left) })},
            KeyCode::S => { event_writer.send(PlayerStateEvent{ state: PlayerAction::Run, direction: Some(Facing::Down) })},
            KeyCode::D => { event_writer.send(PlayerStateEvent{ state: PlayerAction::Run, direction: Some(Facing::Right) })},
            _ => {},
        }
    }
    for key in keyboard_input.get_just_released() {
        state.pressed_keys.remove(key);
        match key {
            KeyCode::W |
            KeyCode::A |
            KeyCode::S |
            KeyCode::D => {
                if state.pressed_keys.is_disjoint(&DIRECTIONAL_KEYS) {
                    event_writer.send(PlayerStateEvent{ state: PlayerAction::Idle, direction: None });
                }
            }
            KeyCode::Space => { event_writer.send(PlayerStateEvent{ state: PlayerAction::Idle, direction: None })},
            _ => {},
        }
    }
}