use bevy::{
    math::Vec2,
    prelude::{
        Component, Entity, GlobalTransform, IntoSystemSetConfig, Plugin, PostUpdate, Query, Res,
        SystemSet,
    },
};
use bevy_rapier2d::prelude::{Collider, PhysicsSet, QueryFilter, RapierContext};

pub struct PhysicsPlugin;

#[derive(Hash, Debug, PartialEq, Eq, Clone, SystemSet)]
struct PostPhysicsSet;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, update_collision_state)
            .configure_set(PostUpdate, PostPhysicsSet.after(PhysicsSet::Writeback));
    }
}

#[derive(Component, Default, Debug)]
pub struct CollisionState {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

fn update_collision_state(
    context: Res<RapierContext>,
    mut collision_states: Query<(Entity, &mut CollisionState, &GlobalTransform, &Collider)>,
) {
    for (entity, mut collision_state, transform, collider) in collision_states.iter_mut() {
        *collision_state = CollisionChecker::new(&context, transform, collider)
            .with_filter(
                QueryFilter::default()
                    .exclude_collider(entity)
                    .exclude_rigid_body(entity),
            )
            .get_collision_state();
    }
}

struct CollisionChecker<'a> {
    context: &'a RapierContext,
    transform: &'a GlobalTransform,
    collider: Collider,
    filter: Option<QueryFilter<'a>>,
}

impl<'a> CollisionChecker<'a> {
    fn new(
        context: &'a RapierContext,
        transform: &'a GlobalTransform,
        collider: &'a Collider,
    ) -> Self {
        let mut checker = Self {
            context,
            transform,
            collider: collider.clone(),
            filter: None,
        };
        checker.collider.set_scale(Vec2::new(0.99, 0.99), 16);
        checker
    }

    fn with_filter(self, filter: QueryFilter<'a>) -> Self {
        Self {
            filter: Some(filter),
            ..self
        }
    }

    fn check_collision(&self, direction: Vec2) -> bool {
        self.context
            .cast_shape(
                self.transform.translation().truncate(),
                0.0,
                direction,
                &self.collider,
                0.1,
                self.filter.unwrap_or_default(),
            )
            .is_some()
    }

    fn get_collision_state(&self) -> CollisionState {
        CollisionState {
            top: self.check_collision(Vec2::new(0.0, 1.0)),
            bottom: self.check_collision(Vec2::new(0.0, -1.0)),
            left: self.check_collision(Vec2::new(-1.0, 0.0)),
            right: self.check_collision(Vec2::new(1.0, 0.0)),
        }
    }
}
