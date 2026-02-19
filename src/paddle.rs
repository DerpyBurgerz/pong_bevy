use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use crate::{Position, Velocity};
use bevy::prelude::{Component, Rectangle};
use crate::collision::{collide_with_slide, Collider, Collision};
use crate::gutter::Gutter;

#[derive(Component, Default)]
pub struct Paddle;
#[derive(Component, Default)]
pub struct Player;
#[derive(Component, Default)]
pub struct Ai;

#[derive(Default, Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    position: Position,
    transform: Transform,
    collider: Collider,
    velocity: Velocity,
}

const PADDLE_SHAPE: Rectangle = Rectangle::new(20., 50.);
const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);
pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let mesh = meshes.add(PADDLE_SHAPE);
    let material = materials.add(PADDLE_COLOR);
    let half_window_size = window.resolution.size() / 2.;
    let padding = 20.;

    let player_position = Vec2::new(-half_window_size.x + padding, 0.);

    commands.spawn((
        Player,
        PaddleBundle {
            position: Position(player_position),
            collider: Collider(PADDLE_SHAPE),
            ..default()
        },
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        ));

    let ai_position = Vec2::new(half_window_size.x - padding, 0.);

    commands.spawn((
        Ai,
        PaddleBundle {
            position: Position(ai_position),
            collider: Collider(PADDLE_SHAPE),
            ..default()
        },
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        ));
}

const PADDLE_SPEED: f32 = 5.;
pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_velocity: Single<&mut Velocity, With<Player>>
) {
    if keyboard_input.pressed(KeyCode::KeyK) {
        paddle_velocity.0.y = PADDLE_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyJ) {
        paddle_velocity.0.y = -PADDLE_SPEED;
    } else {
        paddle_velocity.0.y = 0.;
    }
}

pub fn move_paddles(mut paddles: Query<(&mut Position, &Velocity), With<Paddle>>) {
    for (mut position, velocity) in &mut paddles {
        position.0 += velocity.0;
    }
}

pub fn constrain_paddle_position(
    mut paddles: Query<
        (&mut Position, &Collider),
        (With<Paddle>, Without<Gutter>),
    >, gutters: Query<(&Position, &Collider), (With<Gutter>, Without<Paddle>)>
) {
    for (mut paddle_position, paddle_collider) in &mut paddles {
        for (gutter_position, gutter_collider) in &gutters {
            let paddle_aabb = Aabb2d::new(paddle_position.0, paddle_collider.half_size());
            let gutter_aabb = Aabb2d::new(gutter_position.0, gutter_collider.half_size());

            if let Some(collision) = collide_with_slide(paddle_aabb, gutter_aabb) {
                match collision {
                    Collision::Top => {
                        paddle_position.0.y = gutter_position.0.y
                            + gutter_collider.half_size().y
                            + paddle_collider.half_size().y;
                    }
                    Collision::Bottom => {
                        paddle_position.0.y = gutter_position.0.y
                            - gutter_collider.half_size().y
                            - paddle_collider.half_size().y;
                    }
                    _ => {}
                }
            }
        }
    }

}