use bevy::prelude::*;
use crate::system::{AnimationParams, start_animation, load_animation};
use std::collections::HashMap;

pub struct Player;

pub struct PlayerAnimations(HashMap<(PlayerAction, Facing), AnimationParams>);

// On startup, load all the animations we're going to use, plus add the Player entity.
pub fn player_startup(
    mut commands: Commands,
    loader: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>
) {
    let mut x = HashMap::new();
    x.insert((PlayerAction::Idle, Facing::Up),    AnimationParams::new("texture/player/back/idle_sprite_sheet.png",  293, 337, 5, 5, false, false, 25));
    x.insert((PlayerAction::Idle, Facing::Down),  AnimationParams::new("texture/player/front/idle_sprite_sheet.png", 280, 339, 5, 5, false, false, 25));
    x.insert((PlayerAction::Idle, Facing::Left),  AnimationParams::new("texture/player/left/idle_sprite_sheet.png",  267, 338, 5, 5, false, false, 25));
    x.insert((PlayerAction::Idle, Facing::Right), AnimationParams::new("texture/player/left/idle_sprite_sheet.png",  267, 338, 5, 5, true,  false, 25));
    x.insert((PlayerAction::Run,  Facing::Up),    AnimationParams::new("texture/player/back/run_sprite_sheet.png",   286, 341, 4, 4, false, false, 25));
    x.insert((PlayerAction::Run,  Facing::Down),  AnimationParams::new("texture/player/front/run_sprite_sheet.png",  286, 341, 4, 4, false, false, 25));
    x.insert((PlayerAction::Run,  Facing::Left),  AnimationParams::new("texture/player/left/run_sprite_sheet.png",   269, 341, 4, 4, false, false, 25));
    x.insert((PlayerAction::Run,  Facing::Right), AnimationParams::new("texture/player/left/run_sprite_sheet.png",   269, 341, 4, 4, true,  false, 25));

    for (_, mut animation) in x.iter_mut() {
        load_animation(&mut animation, &*loader, &mut *atlases);
    }
    
    commands.insert_resource::<PlayerAnimations>(PlayerAnimations(x));
    commands
        .spawn()
        .insert(Player);
}

// Which direction is the player facing?
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Facing {
    Up, Down, Left, Right,
}

// What is the player doing?
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PlayerAction {
    Idle,
    Run,
}

// An event that signals a change in the PlayerAction; fired off by the input system.
pub struct PlayerStateEvent {
    pub state: PlayerAction,
    pub direction: Option<Facing>,
}

#[derive(Debug)]
pub struct PlayerState {
    direction: Facing,
}
impl Default for PlayerState {
    fn default() -> Self {
        PlayerState { direction: Facing::Down }
    }
}

// A system to respond to PlayerStateEvents by starting the corresponding animation.
pub fn player_state_event_receiver(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    player_anims: Res<PlayerAnimations>,
    mut state: Local<PlayerState>,
    mut event_reader: EventReader<PlayerStateEvent>,
) {
    if let Some(player) = player_query.iter().next() {
        for e in event_reader.iter() {
            if let Some(new_direction) = e.direction { state.direction = new_direction; }
            if let Some(animation_params) = player_anims.0.get(&(e.state, state.direction)) {
                start_animation(&mut commands, player, animation_params);
            } else {
                eprintln!("No animation for {:?}", (e.state, state.direction));
            }
        }
    }
}