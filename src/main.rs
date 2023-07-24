use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_asset_loader::standard_dynamic_asset::StandardDynamicAssetCollection;
use bevy_rapier2d::prelude::*;
use controls::{FrogActions, InputPlugin};
use gameplay::{FrogAssets, FrogBundle, GameplayPlugin};
use leafwing_input_manager::prelude::InputManagerPlugin;

mod controls;
mod gameplay;

pub const PIXELS_PER_METER: f32 = 16.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jumpy Frog!".into(),
                resolution: (640., 360.).into(),
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            InputManagerPlugin::<FrogActions>::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0),
            RapierDebugRenderPlugin::default(),
            InputPlugin,
            GameplayPlugin,
        ))
        .add_state::<GameStates>()
        .add_loading_state(
            LoadingState::new(GameStates::LoadLevel).continue_to_state(GameStates::PlayLevel),
        )
        .add_collection_to_loading_state::<_, FrogAssets>(GameStates::LoadLevel)
        .add_systems(Startup, setup_graphics)
        .add_systems(OnEnter(GameStates::LoadLevel), setup_asset_loading)
        .add_systems(OnEnter(GameStates::PlayLevel), setup_level)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(LevelInfo {
        frog: "basic".into(),
        level: "blocks".into(),
    });
}

#[derive(Resource)]
struct LevelInfo {
    frog: String,
    level: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameStates {
    Menu,
    #[default]
    LoadLevel,
    PlayLevel,
}

fn setup_asset_loading(
    mut dynamic_asset_collections: ResMut<DynamicAssetCollections<GameStates>>,
    level_info: Res<LevelInfo>,
) {
    info!(
        "Loading assets: {}",
        format!("frogs/{}.assets.ron", level_info.frog)
    );
    dynamic_asset_collections.register_file::<StandardDynamicAssetCollection>(
        GameStates::LoadLevel,
        &format!("frogs/{}.assets.ron", level_info.frog),
    );
}

fn setup_level(
    mut commands: Commands,
    frog_assets: Res<FrogAssets>,
    mut config: ResMut<RapierConfiguration>,
) {
    /* Create the room. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -180.0, 0.0)));
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 180.0, 0.0)));

    commands
        .spawn(Collider::cuboid(50.0, 360.0))
        .insert(TransformBundle::from(Transform::from_xyz(320.0, 0.0, 0.0)));
    commands
        .spawn(Collider::cuboid(50.0, 360.0))
        .insert(TransformBundle::from(Transform::from_xyz(-320.0, 0.0, 0.0)));

    config.gravity = Vec2::new(0.0, -500.0);

    commands
        .spawn(FrogBundle::new(
            Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
            &frog_assets,
        ))
        .insert(FrogActions::get_default_input_manager_bundle());
}
