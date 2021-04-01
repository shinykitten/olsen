use bevy::{
    prelude::*,
    render::pass::ClearColor,
};

mod system;
use crate::system::*;

fn main() {
    App::build()
        // Resources.
        .insert_resource(ClearColor(Color::rgb(0.675, 0.882, 0.686)))
        .insert_resource(WindowDescriptor {
            title: "Olsen".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })

        // Startup systems.
        .add_startup_system(setup.system())
        .add_startup_system(player_startup.system())

        // Systems.
        .add_system(input_event_receiver.system())
        .add_system(player_state_event_receiver.system())
        .add_system(sprite_animator.system())

        // Events.
        .add_event::<PlayerStateEvent>()

        // Plugins.
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}