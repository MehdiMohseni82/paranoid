use bevy::{math::vec3, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WallsPlugin;

#[derive(Bundle)]
struct WallBundle {
    shape: ShapeBundle,
    collider: Collider,
    rigid_body: RigidBody,
    fill: Fill,
}

impl WallBundle {
    pub fn new(location: Vec2, size: Vec2, color: Color) -> WallBundle {
        let shape = shapes::Rectangle {
            extents: Vec2::new(size.x, size.y),
            origin: shapes::RectangleOrigin::Center,
        };

        WallBundle {
            shape: ShapeBundle {
                transform: Transform {
                    translation: location.extend(0.0),
                    scale: Vec3::ONE,
                    ..default()
                },
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            collider: Collider::cuboid(shape.extents.x / 2.0, shape.extents.y / 2.0),
            rigid_body: RigidBody::Fixed,
            fill: Fill::color(color),
        }
    }
}

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls);
    }
}

#[derive(Component)]
pub struct BottomWall;

fn spawn_walls(mut commands: Commands) {
    //Spawn bottom wall
    commands.spawn(WallBundle::new(
        Vec2::new(0.0, crate::PIXELS_PER_METER * crate::GAME_BOX_HEIGHT / -2.0),
        Vec2::new(
            crate::PIXELS_PER_METER * crate::GAME_BOX_WIDTH,
            crate::PIXELS_PER_METER * crate::WALL_THIKNESS_FACTOR,
        ),
        Color::RED,
    ));

    //Spawn top wall
    commands.spawn(WallBundle::new(
        Vec2::new(0.0, crate::PIXELS_PER_METER * crate::GAME_BOX_HEIGHT / 2.0),
        Vec2::new(
            crate::PIXELS_PER_METER * crate::GAME_BOX_WIDTH,
            crate::PIXELS_PER_METER * crate::WALL_THIKNESS_FACTOR,
        ),
        Color::RED,
    ));

    //Spawn right wall
    commands.spawn(WallBundle::new(
        Vec2::new((crate::GAME_BOX_WIDTH / 2.0) * crate::PIXELS_PER_METER, 0.0),
        Vec2::new(
            crate::PIXELS_PER_METER * crate::WALL_THIKNESS_FACTOR,
            crate::PIXELS_PER_METER * (crate::GAME_BOX_HEIGHT + crate::WALL_THIKNESS_FACTOR),
        ),
        Color::RED,
    ));

    //Spawn right wall
    commands.spawn(WallBundle::new(
        Vec2::new(
            (crate::GAME_BOX_WIDTH / -2.0) * crate::PIXELS_PER_METER,
            0.0,
        ),
        Vec2::new(
            crate::PIXELS_PER_METER * crate::WALL_THIKNESS_FACTOR,
            crate::PIXELS_PER_METER * (crate::GAME_BOX_HEIGHT + crate::WALL_THIKNESS_FACTOR),
        ),
        Color::RED,
    ));
}
