use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAsssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAsssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAsssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAsssets {
        asteroid: asset_server.load("Planet.glb#Scene0"),
        spaceship: asset_server.load("Spaceship.glb#Scene0"),
        missiles: asset_server.load("Bullets Pickup.glb#Scene0"),
    };
}
