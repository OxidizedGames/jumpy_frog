use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use controls::{FrogActions, InputPlugin};
use gameplay::{FrogBundle, GameplayPlugin};
use leafwing_input_manager::prelude::InputManagerPlugin;

mod controls;
mod gameplay;

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
        .add_systems(Startup, (setup_graphics, setup_physics))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
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
            asset_server.load("Frog.png"),
        ))
        .insert(FrogActions::get_default_input_manager_bundle());
}
