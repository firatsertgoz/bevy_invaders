use bevy::prelude::*;
pub struct MovementPlugin;

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub(crate) velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
}

#[derive(Component, Debug)]
pub(crate) struct Velocity {
    pub(crate) value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub(crate) value: Vec3,
}
impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity));
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
