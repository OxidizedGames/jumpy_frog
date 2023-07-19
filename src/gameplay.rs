use bevy::{
    prelude::{
        info, Bundle, Camera, Commands, Component, Entity, GlobalTransform, Handle, Image,
        IntoSystemConfigs, Plugin, Query, Transform, Update, Without,
    },
    sprite::SpriteBundle,
};
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody, Velocity};
use leafwing_input_manager::prelude::ActionState;

use crate::controls::{FrogActions, FrogAimState};

use self::physics::{CollisionState, PhysicsPlugin};

mod physics;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(PhysicsPlugin)
            .add_systems(Update, (start_aim, update_aim, finish_aim).chain());
    }
}

#[derive(Bundle)]
pub struct FrogBundle {
    rigid_body: RigidBody,
    collider: Collider,
    jump_strength: JumpStrength,
    impulse: Velocity,
    locked_axes: LockedAxes,
    collision_state: CollisionState,
    sprite: SpriteBundle,
}

impl FrogBundle {
    pub fn new(transform: Transform, texture: Handle<Image>) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(8.0, 8.0),
            jump_strength: JumpStrength(10.0),
            impulse: Velocity::default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collision_state: CollisionState::default(),
            sprite: SpriteBundle {
                texture,
                transform,
                ..Default::default()
            },
        }
    }
}

#[derive(Component)]
pub struct JumpStrength(pub f32);

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

type FinishAimComponents<'a> = (
    Entity,
    &'a ActionState<FrogActions>,
    &'a FrogAimState,
    &'a JumpStrength,
    &'a CollisionState,
    &'a mut Velocity,
);

fn finish_aim(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut frogs: Query<FinishAimComponents>,
) {
    let (camera, camera_transform) = camera_query.single();
    for (entity, action_state, aim_state, jump_strength, collision_state, mut velocity) in
        frogs.iter_mut()
    {
        if action_state.just_pressed(FrogActions::AimRelease) {
            if collision_state.bottom {
                let start_pos = camera
                    .viewport_to_world_2d(camera_transform, aim_state.start_pos)
                    .unwrap();
                let end_pos = camera
                    .viewport_to_world_2d(camera_transform, aim_state.end_pos)
                    .unwrap();
                let direction = start_pos - end_pos;
                velocity.linvel = direction.clamp_length_max(32.0) * jump_strength.0;
                info!("Launching Frog with impulse: {}", velocity.linvel);
            }
            commands.entity(entity).remove::<FrogAimState>();
        }
        if action_state.just_pressed(FrogActions::AimCancel) {
            commands.entity(entity).remove::<FrogAimState>();
            info!("Cancelling Frog Launch");
        }
    }
}
