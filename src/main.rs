pub(crate) mod camera;
mod compute;
pub(crate) mod env;
pub(crate) mod input;
pub(crate) mod instance;
pub(crate) mod lorenz;
pub(crate) mod render;
pub(crate) mod state;
pub(crate) mod vertex;
pub(crate) mod texture;

use camera::Camera;
use compute::ComputeState;
use env::Environment;
use lorenz::{LorenzConfig, LorenzState};
use pollster::FutureExt;
use render::RenderState;
use state::State;
use winit::event_loop::EventLoop;


fn main() {
    let event_loop = EventLoop::new();

    let env = Environment::new(&event_loop).block_on();

    let lorenz_state = LorenzState::new(LorenzConfig::default());

    let (camera, camera_bind_group_layout) = Camera::create_camera(&env.device, &env.config);

    let render_state = RenderState::new(&lorenz_state, &env, camera_bind_group_layout);

    let compute_state = ComputeState::new(
        &env.device,
        &render_state.instances.buffer,
        lorenz_state.lorenz_config,
    );
    let state = State {
        env,
        render_state,
        lorenz_state,
        camera,
        compute_state,
    };

    state.run(event_loop);
}
