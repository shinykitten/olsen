use bevy::prelude::*;
use bevy::utils::Duration;

/*
How to use:

1. App::build().add_system(sprite_animator.system());
2. Call load_animation once per animation.
3. Pass the same AnimationParams to start_animation to start that animation on a particular Entity.
*/

// A timer attached to any Entity running an animation.
pub struct AnimationTimer(pub Timer);

// A system to animate sprites.
//
// Every animated sprite has an AnimationTimer, a sprite sheet (TextureAtlas), and its current sprite (TextureAtlasSprite).
// This system ticks the timer and updates the current sprite's index into the sprite sheet.
pub fn sprite_animator(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut AnimationTimer, &Handle<TextureAtlas>, &mut TextureAtlasSprite)>,
) {
    for (mut timer, texture_atlas_handle, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            match texture_atlases.get(texture_atlas_handle) {
                Some(texture_atlas) => {
                    sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
                },
                None => {
                    eprintln!("AnimatedSprite missing TextureAtlas asset: {:?}", texture_atlas_handle);
                }
            }
        }
    }
}

// AnimationParams specify all the parameters for a spritesheet-based animation.
#[derive(Clone, Default, Hash)]
pub struct AnimationParams {
    sheet_file: String,
    cell_width: usize,
    cell_height: usize,
    grid_width: usize,
    grid_height: usize,
    flip_x: bool,
    flip_y: bool,
    tick_rate_ms: u64,

    // Not an animation parameter.  Will be populated by load_animation.
    atlas_handle: Option<Handle<TextureAtlas>>,
}
impl AnimationParams {
    pub fn new(
        sheet_file: &str,
        cell_width: usize, cell_height: usize,
        grid_width: usize, grid_height: usize,
        flip_x: bool, flip_y: bool,
        tick_rate_ms: u64,
    ) -> Self {
        AnimationParams {
            sheet_file: sheet_file.to_string(), atlas_handle: None,
            cell_width, cell_height, grid_width, grid_height, flip_x, flip_y, tick_rate_ms
        }
    }
}

// Call once per animation; loads the assets.
// Stores a handle to the TextureAtlas on the AnimationParams for later use by start_animation.
pub fn load_animation(
    mut params: &mut AnimationParams,
    loader: &AssetServer,
    asset_mgr: &mut Assets<TextureAtlas>
) {
    let texture_handle = loader.load(params.sheet_file.as_str());
    params.atlas_handle = Some(asset_mgr.add(TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(params.cell_width as f32, params.cell_height as f32),
        params.grid_width,
        params.grid_height)));
}

// Starts the animation on the given Entity.
// Expects the same AnimationParams that was given to load_animation.
pub fn start_animation(
    commands: &mut Commands,
    entity: Entity,
    params: &AnimationParams,
) {
    if let Some(handle) = &params.atlas_handle {
        commands
            .entity(entity)
            .insert_bundle(SpriteSheetBundle{
                texture_atlas: handle.clone(),
                sprite: TextureAtlasSprite {
                    flip_x: params.flip_x,
                    flip_y: params.flip_y,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(AnimationTimer(Timer::new(Duration::from_millis(params.tick_rate_ms), true)));
    }
}