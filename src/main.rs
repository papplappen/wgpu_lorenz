pub(crate) mod camera;
pub(crate) mod env;
pub(crate) mod instance;
pub(crate) mod lorenz;
pub(crate) mod render;
pub(crate) mod state;
pub(crate) mod vertex;

use camera::Camera;
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

    let state = State {
        env,
        render_state,
        lorenz_state,
        camera,
    };

    state.run(event_loop);
}
