use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{default, Bundle, Circle, ColorMaterial, Commands, Component, MeshMaterial2d, On, Rectangle, ResMut, Single, Transform, With};
use crate::{Position, Scored, Velocity};
use crate::collision::Collider;

const BALL_SIZE:f32 = 10.;
const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_COLOR: Color = Color::srgb(1., 0., 0.);
const BALL_SPEED: f32 = 2.;
pub fn spawn_ball(mut commands: Commands,
              mut meshes: ResMut<Assets<Mesh>>,
              mut materials: ResMut<Assets<ColorMaterial>>) {
    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);

    commands.spawn((BallBundle{
        velocity: Velocity(Vec2{ x: BALL_SPEED, y: 0.}),
        collider: Collider(Rectangle::new(BALL_SIZE, BALL_SIZE)),
        ..default()
    }, Mesh2d(mesh), MeshMaterial2d(material)));
}
pub fn move_ball(ball : Single<(&mut Position, &Velocity), With<Ball>>) {
    let (mut pos, vel) = ball.into_inner();
    pos.0 += vel.0;
}

#[derive(Component, Default)]
#[require(Position)]
pub struct Ball;

#[derive(Bundle, Default)]
struct BallBundle {
    ball: Ball,
    position: Position,
    transform: Transform,
    velocity: Velocity,
    collider: Collider,
}
pub fn reset_ball(
    _event: On<Scored>,
    ball: Single<(&mut Position, &mut Velocity), With<Ball>>,
) {
    let (mut ball_position, mut ball_velocity) = ball.into_inner();
    ball_position.0 = Vec2::ZERO;
    ball_velocity.0 = Vec2::new(BALL_SPEED, 0.);
}