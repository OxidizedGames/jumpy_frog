use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use controls::{FrogActions, FrogAimState, InputPlugin};
use gameplay::{FrogBundle, JumpStrength};
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};

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
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_systems((start_aim, update_aim, finish_aim).chain())
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

fn start_aim(
    mut commands: Commands,
    frogs: Query<(Entity, &ActionState<FrogActions>), Without<FrogAimState>>,
) {
    for (entity, action_state) in frogs.iter() {
        if action_state.just_pressed(FrogActions::Aim) {
            if let Some(cursor_pos) = action_state.axis_pair(FrogActions::AimPositionScreen) {
                commands
                    .entity(entity)
                    .insert(FrogAimState::new(cursor_pos.xy()));
                info!("Starting Frog Aim: {}", cursor_pos.xy());
            }
        }
    }
}

fn update_aim(mut frogs: Query<(&ActionState<FrogActions>, &mut FrogAimState)>) {
    for (action_state, mut aim_state) in frogs.iter_mut() {
        if action_state.pressed(FrogActions::Aim) {
            if let Some(cursor_pos) = action_state.axis_pair(FrogActions::AimPositionScreen) {
                aim_state.end_pos = cursor_pos.xy();
                info!("Updating Frog Aim: {}", aim_state.end_pos);
            }
        }
    }
}

fn finish_aim(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut frogs: Query<(
        Entity,
        &ActionState<FrogActions>,
        &FrogAimState,
        &JumpStrength,
        &mut ExternalImpulse,
    )>,
) {
    let (camera, camera_transform) = camera_query.single();
    for (entity, action_state, aim_state, jump_strength, mut impulse) in frogs.iter_mut() {
        if action_state.just_pressed(FrogActions::AimRelease) {
            let start_pos = camera
                .viewport_to_world_2d(camera_transform, aim_state.start_pos)
                .unwrap();
            let end_pos = camera
                .viewport_to_world_2d(camera_transform, aim_state.end_pos)
                .unwrap();
            let direction = start_pos - end_pos;
            impulse.impulse = direction.clamp_length_max(32.0) * jump_strength.0;
            info!("Launching Frog with impulse: {}", impulse.impulse);
            commands.entity(entity).remove::<FrogAimState>();
        }
        if action_state.just_pressed(FrogActions::AimCancel) {
            commands.entity(entity).remove::<FrogAimState>();
            info!("Cancelling Frog Launch");
        }
    }
}
