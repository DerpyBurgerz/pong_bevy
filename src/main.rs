use bevy::math::bounding::{
    Aabb2d,
    BoundingVolume,
    IntersectsVolume
};

mod paddle;
mod ball;
mod collision;
mod gutter;

use bevy::prelude::*;
use crate::ball::{move_ball, reset_ball, spawn_ball, Ball};
use crate::collision::{handle_collisions, Collider};
use crate::gutter::spawn_gutter;
use crate::paddle::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_ball,
            spawn_camera,
            spawn_paddles,
            spawn_gutter,
        ))
        .insert_resource(Score { player: 0, ai: 0 })
        .add_systems(Update, (
            move_ball.before(project_positions),
            handle_collisions.after(move_ball),

            handle_player_input.before(project_positions),
            move_paddles.before(project_positions),

            constrain_paddle_position.after(move_paddles),
            detect_goal.after(move_ball),

            project_positions,
            ))
        .add_observer(reset_ball)
        .add_observer(update_score)
        .run();
}

fn spawn_camera(
    mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>){
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

#[derive(Component, Default)]
struct Velocity(Vec2);
#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Resource)]
struct Score {
    player: u32,
    ai: u32
}

#[derive(EntityEvent)]
struct Scored {
    #[event_target]
    scorer: Entity
}
fn detect_goal(
    ball: Single<(&Position, &Collider), With<Ball>>,
    player: Single<Entity, (With<Player>, Without<Ai>)>,
    ai: Single<Entity, (With<Ai>, Without<Player>)>,
    window: Single<&Window>,
    mut commands: Commands,
) {
    let (ball_position, ball_collider) = ball.into_inner();
    let half_window_size = window.resolution.size() / 2.;

    if ball_position.0.x - ball_collider.half_size().x > half_window_size.x {
        commands.trigger(Scored { scorer: *player });
    }

    if ball_position.0.x + ball_collider.half_size().x < -half_window_size.x {
        commands.trigger(Scored { scorer: *ai });
    }
}

fn update_score(
    event: On<Scored>,
    mut score: ResMut<Score>,
    is_ai: Query<&Ai>,
    is_player: Query<&Player>,
) {
    if is_ai.get(event.scorer).is_ok() {
        score.ai += 1;
        info!("AI scored! {} - {}", score.player, score.ai);
    }

    if is_player.get(event.scorer).is_ok() {
        score.player += 1;
        info!("Player scored! {} - {}", score.player, score.ai);
    }
}
