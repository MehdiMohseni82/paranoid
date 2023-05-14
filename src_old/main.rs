use bevy::{prelude::*, window::PresentMode};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

const PIXELS_PER_METER: f32 = 492.3;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const GAP_BETWEEN_PADDLE_AND_WALL: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_SPEED: f32 = 400.0;
const INITIAL_TOP_PLAYER_BALL_DIRECTION: Vec2 = Vec2::new(0.5, 0.5);
const INITIAL_BOTTOM_PLAYER_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = 0.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -450.;
const TOP_WALL: f32 = 450.;

const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
// These values are exact
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 210.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These values are lower bounds, as the number of bricks is computed
// const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pinball2d".into(),
                resolution: (1000., 1000.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Once)))
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        // Add our gameplay simulation systems to the fixed timestep schedule
        // .add_systems(
        //     (
        //         check_for_collisions,
        //         apply_velocity.before(check_for_collisions),
        //         move_paddle
        //             .before(check_for_collisions)
        //             .after(apply_velocity),
        //         play_collision_sound.after(check_for_collisions),
        //     )
        //         .in_schedule(CoreSchedule::FixedUpdate),
        // )
        // Configure how frequently our gameplay systems are run
        // .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        // .add_system(kill_ball)
        // .add_system(spawn_ball)
        // .add_system(handle_input)
        // .add_system(update_scoreboard)
        // .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Paddle {
    name: String,
}

#[derive(Component)]
struct TopPlayer;

#[derive(Component)]
struct BottomPlayer;

#[derive(Component)]
struct Ball {
    name: String,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

struct MovementEvent {
    movement: Vec2,
    player: Entity,
}

#[derive(Component)]
struct Brick;

#[derive(Resource)]
struct CollisionSound(Handle<AudioSource>);

#[derive(Resource)]
struct SpawnTimer(Timer);

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    shape: ShapeBundle,
    collider: Collider,
    // transform: Transform,
    rigid_body: RigidBody,
    fill: Fill,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        let shape_top_and_bottom_wall = shapes::Rectangle {
            extents: Vec2::new(PIXELS_PER_METER * 5000f32, PIXELS_PER_METER * 5000f32),
            origin: shapes::RectangleOrigin::Center,
        };

        WallBundle {
            shape: ShapeBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                path: GeometryBuilder::build_as(&shape_top_and_bottom_wall),
                ..default()
            },
            collider: Collider,
            rigid_body: RigidBody::Fixed,
            fill: Fill::color(Color::RED),
        }
    }
}

// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

// Add the game's entities to our world
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    //Spawn left wall

    let shape_top_and_bottom_wall = shapes::Rectangle {
        extents: Vec2::new(
            crate::PIXELS_PER_METER * 0.73,
            crate::PIXELS_PER_METER * 0.03,
        ),
        origin: shapes::RectangleOrigin::Center,
    };

    let top_wall_pos = Vec2::new(0.0, crate::PIXELS_PER_METER * 0.64);
    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape_top_and_bottom_wall),
                ..default()
            },
            Fill::color(Color::TEAL),
        ))
        .insert(RigidBody::Fixed)
        .insert(Transform::from_xyz(top_wall_pos.x, top_wall_pos.y, 0.0));

    println!("HELLO")
}
