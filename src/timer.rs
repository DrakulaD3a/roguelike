use bevy::prelude::*;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<TickState>()
            .add_startup_system(create_tick_timer)
            .add_system(tick);
    }
}

#[derive(Resource)]
pub struct TickTimer(Timer);

#[derive(States, Eq, Hash, Debug, Clone, PartialEq, Default)]
pub enum TickState {
    #[default]
    Input,
    Execution,
}

const TICK_TIME: f32 = 5.0;

fn create_tick_timer(mut commands: Commands) {
    commands.insert_resource(TickTimer(Timer::from_seconds(
        TICK_TIME,
        TimerMode::Repeating,
    )));
}

fn tick(
    mut tick_state: ResMut<NextState<TickState>>,
    time: Res<Time>,
    mut tick_timer: ResMut<TickTimer>,
) {
    if tick_timer.0.tick(time.delta()).finished() {
        tick_state.set(TickState::Execution);
    }
}
