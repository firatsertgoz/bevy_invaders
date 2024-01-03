use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAsssets,
    collision_detection::Collider,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const ROTATE_SPEED: f32 = 2.5;
const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 5.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANZE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const RADIUS: f32 = 1.0;
#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_asteroid)
        .add_systems(Update, rotate_asteroid_rotation);
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAsssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANZE_Z),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize();
    let velocity = random_unit_vector();
    let acceleration = random_unit_vector();

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            collider: Collider::new(RADIUS),
            acceleration: Acceleration::new(acceleration),
            model: SceneBundle {
                scene: scene_assets.asteroid.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
        },
        Asteroid,
    ));
}

fn rotate_asteroid_rotation(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_seconds());
    }
}
