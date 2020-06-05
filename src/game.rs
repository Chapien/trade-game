extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

pub const MAP_HEIGHT: f32 = 100.0;
pub const MAP_WIDTH: f32 = 100.0;
pub struct Game;


impl SimpleState for Game {
    // Called when a state is initialized.
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions to initialize the camera.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);
    }
}

fn init_camera(world : &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen.
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

