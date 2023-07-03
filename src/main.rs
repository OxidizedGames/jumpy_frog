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
        .add_plugin(InputManagerPlugin::<FrogActions>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InputPlugin)
        .add_plugin(GameplayPlugin)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands, asset_server: ResMut<AssetServer>) {
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

    commands
        .spawn(FrogBundle::new(
            Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
            asset_server.load("Frog.png"),
        ))
        .insert(FrogActions::get_default_input_manager_bundle());
}
