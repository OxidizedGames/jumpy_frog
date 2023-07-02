use bevy::{
    input::InputSystem,
    prelude::{
        App, Camera, Component, CoreSet, GlobalTransform, IntoSystemConfig, IntoSystemConfigs,
        MouseButton, Plugin, Query, Res, Resource, Vec2,
    },
    window::Window,
};
use leafwing_input_manager::{
    axislike::DualAxisData,
    plugin::InputManagerSystem,
    prelude::{ActionState, InputMap},
    systems::run_if_enabled,
    Actionlike, InputManagerBundle,
};

pub const DEFAULT_AIM_CANCEL_TIME: u128 = 300;

pub const DEFAULT_AIM_CANCEL_DISTANCE: f32 = 30.0;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                update_cursor_state_from_window
                    .run_if(run_if_enabled::<FrogActions>)
                    .in_base_set(CoreSet::PreUpdate)
                    .in_set(InputManagerSystem::ManualControl)
                    .before(InputManagerSystem::ReleaseOnDisable)
                    .after(InputManagerSystem::Tick)
                    .after(InputManagerSystem::Update)
                    .after(InputSystem),
                update_aiming
                    .run_if(run_if_enabled::<FrogActions>)
                    .in_base_set(CoreSet::PreUpdate)
                    .in_set(InputManagerSystem::ManualControl)
                    .before(InputManagerSystem::ReleaseOnDisable)
                    .after(InputManagerSystem::Tick)
                    .after(InputManagerSystem::Update)
                    .after(InputSystem),
            )
                .chain(),
        )
        .insert_resource(AimCancelTime(DEFAULT_AIM_CANCEL_TIME))
        .insert_resource(AimCancelDistance(DEFAULT_AIM_CANCEL_DISTANCE));
    }
}

#[derive(Resource)]
pub struct AimCancelTime(u128);

#[derive(Resource)]
pub struct AimCancelDistance(f32);

#[derive(Component)]
pub struct FrogAimState {
    pub start_pos: Vec2,
    pub end_pos: Vec2,
}

impl FrogAimState {
    pub fn new(start_pos: Vec2) -> Self {
        Self {
            start_pos,
            end_pos: start_pos,
        }
    }

    pub fn length(&self) -> f32 {
        (self.start_pos - self.end_pos).length()
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum FrogActions {
    Aim,
    AimPositionWorld,
    AimPositionScreen,
    AimCancel,
    AimRelease,
}

impl FrogActions {
    pub fn get_default_input_manager_bundle() -> InputManagerBundle<Self> {
        InputManagerBundle::<FrogActions> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(MouseButton::Left, FrogActions::Aim)
                .insert(MouseButton::Right, FrogActions::AimCancel)
                .build(),
        }
    }
}

fn update_cursor_state_from_window(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut action_state_query: Query<&mut ActionState<FrogActions>>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    let screen_position = window.cursor_position();

    let world_position = screen_position
        .and_then(|pos| camera.viewport_to_world_2d(camera_transform, pos))
        .map(DualAxisData::from_xy);

    let screen_position = screen_position.map(DualAxisData::from_xy);

    for mut action_state in action_state_query.iter_mut() {
        action_state
            .action_data_mut(FrogActions::AimPositionScreen)
            .axis_pair = screen_position;

        action_state
            .action_data_mut(FrogActions::AimPositionWorld)
            .axis_pair = world_position;
    }
}

fn update_aiming(
    mut action_state_query: Query<(&mut ActionState<FrogActions>, &FrogAimState)>,
    aim_cancel_time: Res<AimCancelTime>,
    aim_cancel_distance: Res<AimCancelDistance>,
) {
    for (mut frog_action, aim_state) in action_state_query.iter_mut() {
        if frog_action.pressed(FrogActions::AimCancel) {
            frog_action.consume(FrogActions::Aim);
        }

        if frog_action.just_released(FrogActions::Aim)
            && !frog_action.pressed(FrogActions::AimCancel)
        {
            if frog_action.previous_duration(FrogActions::Aim).as_millis() < aim_cancel_time.0
                && aim_state.length() < aim_cancel_distance.0
            {
                frog_action.press(FrogActions::AimCancel)
            } else {
                frog_action.press(FrogActions::AimRelease)
            }
        }
    }
}
