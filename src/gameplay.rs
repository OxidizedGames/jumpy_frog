use bevy::{
    prelude::{Bundle, Component, Handle, Image, Transform},
    sprite::SpriteBundle,
};
use bevy_rapier2d::prelude::{Collider, ExternalImpulse, RigidBody};

#[derive(Bundle)]
pub struct FrogBundle {
    rigid_body: RigidBody,
    collider: Collider,
    sprite: SpriteBundle,
    jump_strength: JumpStrength,
    impulse: ExternalImpulse,
}

impl FrogBundle {
    pub fn new(transform: Transform, texture: Handle<Image>) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(8.0, 8.0),
            sprite: SpriteBundle {
                texture,
                transform,
                ..Default::default()
            },
            jump_strength: JumpStrength(10.0),
            impulse: ExternalImpulse::default(),
        }
    }
}

#[derive(Component)]
pub struct JumpStrength(pub f32);
