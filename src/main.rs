use bevy::{prelude::*, window::PresentMode};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

mod ball;
use ball::*;

mod walls;
use walls::*;

mod bricks;
use bricks::*;

pub const PIXELS_PER_METER: f32 = 492.3;
pub const SCREEN_RESOLUTION: Vec2 = Vec2::new(1200.0, 800.0);
pub const WALL_THIKNESS_FACTOR: f32 = 0.03;
pub const GAME_BOX_WIDTH: f32 = 1.84;
pub const GAME_BOX_HEIGHT: f32 = 1.5;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pinball2d".into(),
                resolution: (SCREEN_RESOLUTION.x, SCREEN_RESOLUTION.y).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WallsPlugin)
        // .add_plugin(LauncherPlugin)
        // .add_plugin(FlippersPlugin)
        // .add_plugin(BallPlugin)
        .add_plugin(BricksPlugin)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // Set gravity to x and spawn camera.
    //rapier_config.gravity = Vector2::zeros();
    rapier_config.gravity = Vec2::new(0.0, -520.0);

    commands.spawn(Camera2dBundle::default());
}
