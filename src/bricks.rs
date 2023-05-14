use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BricksPlugin;
pub const BRICKS_WIDTH: f32 = 0.12;
pub const BRICKS_HEIGHT: f32 = 0.035;
pub const DISTANCE_BETWEEN_PADDLE_AND_BRICKS: f32 = 0.4;

impl Plugin for BricksPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_pins);
        // .add_system(handle_pin_events)
        // .add_system(respawn_pin_to_toggle_color);
    }
}

#[derive(Component)]
struct Pin {
    timestamp_last_hit: f64,
    position: Vec2,
}

fn spawn_pins(mut commands: Commands) {
    let number_of_bricks_per_row =
        ((crate::GAME_BOX_WIDTH / crate::BRICKS_WIDTH).round() as i32) - 2;
    let ref_point = (crate::GAME_BOX_WIDTH / -2.0) * crate::PIXELS_PER_METER
        + crate::WALL_THIKNESS_FACTOR * crate::PIXELS_PER_METER;
    let briks_distance =
        (crate::PIXELS_PER_METER * crate::BRICKS_WIDTH * 2.0) / (number_of_bricks_per_row as f32);

    let number_of_bricks_row = (((crate::GAME_BOX_HEIGHT
        - (crate::DISTANCE_BETWEEN_PADDLE_AND_BRICKS * 2.0))
        / crate::BRICKS_HEIGHT)
        .round() as i32)
        - 5;

    let briks_vertical_distance = ((crate::PIXELS_PER_METER * crate::BRICKS_HEIGHT * 5.0)
        / (number_of_bricks_row as f32))
        .round() as i32;

    println!("{}", briks_vertical_distance);

    for y in 0..number_of_bricks_row {
        let yyy = ((y as f32) * BRICKS_HEIGHT) * crate::PIXELS_PER_METER * -1.0
            + (0.5 * crate::GAME_BOX_HEIGHT * crate::PIXELS_PER_METER)
            - crate::DISTANCE_BETWEEN_PADDLE_AND_BRICKS * crate::PIXELS_PER_METER;

        for x in 0..number_of_bricks_per_row {
            spawn_single_brick(
                &mut commands,
                Vec2::new(
                    ref_point
                        + (crate::BRICKS_WIDTH * crate::PIXELS_PER_METER * (x as f32))
                        + ((x as f32) * briks_distance),
                    yyy - (y * briks_vertical_distance) as f32,
                ),
                None,
            );
        }
    }
}

fn spawn_single_brick(commands: &mut Commands, position: Vec2, timestamp_last_hit: Option<f64>) {
    let shape_bricks = shapes::Rectangle {
        extents: Vec2::new(
            crate::PIXELS_PER_METER * crate::BRICKS_WIDTH,
            crate::PIXELS_PER_METER * crate::BRICKS_HEIGHT,
        ),
        origin: shapes::RectangleOrigin::TopLeft,
    };

    let temp_timestamp_last_hit = timestamp_last_hit.unwrap_or(0.0);

    let mut color = Color::GREEN;
    if temp_timestamp_last_hit == 0.0 {
        color = Color::TEAL;
    }

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape_bricks),
                ..default()
            },
            Fill::color(Color::BLACK),
            Stroke::new(color, 2.0),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(
            shape_bricks.extents.x / 2.0,
            shape_bricks.extents.y / 2.0,
        ))
        .insert(Transform::from_xyz(position.x, position.y, 0.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Pin {
            timestamp_last_hit: temp_timestamp_last_hit,
            position: position,
        });
}

// fn respawn_pin_to_toggle_color(
//     mut query: Query<(Entity, &Pin), With<Pin>>,
//     time: Res<Time>,
//     mut commands: Commands,
// ) {
//     for (entity, pin) in query.iter_mut() {
//         let diff = time.raw_elapsed_seconds_f64() - pin.timestamp_last_hit;
//         if pin.timestamp_last_hit > 0.0 && diff > 1.0 {
//             //Color have been toggled for more than a second so respawn
//             let pos = pin.position;
//             commands.entity(entity).despawn();
//             spawn_single_pin(&mut commands, pos, None);
//         }
//     }
// }

// fn handle_pin_events(
//     query: Query<(Entity, &Pin), With<Pin>>,
//     time: Res<Time>,
//     mut contact_events: EventReader<CollisionEvent>,
//     mut commands: Commands,
// ) {
//     for contact_event in contact_events.iter() {
//         for (entity, pin) in query.iter() {
//             if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
//                 if h1 == &entity || h2 == &entity {
//                     //Respawn to change color
//                     let pos = pin.position;
//                     let timestamp_last_hit = time.raw_elapsed_seconds_f64();
//                     commands.entity(entity).despawn();
//                     spawn_single_pin(&mut commands, pos, Some(timestamp_last_hit));
//                 }
//             }
//         }
//     }
// }
