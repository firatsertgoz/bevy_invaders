use crate::{asset_loader::SceneAsssets, collision_detection::Collider};
use bevy::prelude::*;

use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

const RADIUS: f32 = 1.0;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEE: f32 = 2.5;
const SHAPESHIP_ROLL_SPEED: f32 = 2.5;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
pub struct SpaceshipPlugin;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;
impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
        app.add_systems(Update, spaceship_movement_controls);
        app.add_systems(Update, spaceship_weapon_controls);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAsssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
            collider: Collider::new(RADIUS),
            acceleration: Acceleration::new(Vec3::ZERO),
        },
        Spaceship,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATION_SPEE * time.delta_seconds();
        roll = SHAPESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::D) {
        rotation = -SPACESHIP_ROTATION_SPEE * time.delta_seconds();
        roll = -SHAPESHIP_ROLL_SPEED * time.delta_seconds();
    }

    // Rotate around the Y axis
    // Ignores the Z-axis rotation applied below.
    transform.rotate_y(rotation);

    transform.rotate_local_z(roll);

    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    scene_assets: Res<SceneAsssets>,
) {
    let transform = query.single();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * 50.0),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
                collider: Collider::new(RADIUS),
                acceleration: Acceleration::new(Vec3::ZERO),
            },
            SpaceshipMissile,
        ));
    }
}
