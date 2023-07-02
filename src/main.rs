pub(crate) mod camera;
mod compute;
mod config;
pub(crate) mod env;
pub(crate) mod input;
pub(crate) mod instance;
pub(crate) mod lorenz;
pub(crate) mod render;
pub(crate) mod state;
pub(crate) mod texture;
pub(crate) mod vertex;

use camera::Camera;
use compute::ComputeState;
use config::{Config, DEFAULT_DELTA_TIME};
use env::Environment;
use lorenz::LorenzState;
use pollster::FutureExt;
use render::RenderState;
use state::State;
use winit::event_loop::EventLoop;

fn main() {
    let config = Config::default();

    let event_loop = EventLoop::new();

    let env = Environment::new(&event_loop).block_on();

    let lorenz_state = LorenzState::new(config.num_lorenz_points);

    let (camera, camera_bind_group_layout) = Camera::create_camera(&env.device, &env.config);

    let render_state = RenderState::new(&lorenz_state, &env, camera_bind_group_layout);

    let compute_state = ComputeState::new(
        &env,
        &render_state.instances.buffer,
        &config,
        DEFAULT_DELTA_TIME,
    );

    let state = State {
        env,
        render_state,
        lorenz_state,
        camera,
        compute_state,
        config,
        delta_time: DEFAULT_DELTA_TIME,
        paused: true,
    };

    state.run(event_loop);
}
