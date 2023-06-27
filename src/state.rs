use std::time::Instant;

use winit::event_loop::EventLoop;

use crate::{
    camera::Camera,
    compute::ComputeState,
    env::Environment,
    input,
    lorenz::{LorenzState, DEFAULT_DELTA_TIME},
    render::RenderState,
};
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};
pub struct State {
    pub env: Environment,
    pub render_state: RenderState,
    pub lorenz_state: LorenzState,
    pub compute_state: ComputeState,
    pub camera: Camera,
}

impl State {
    pub fn run(mut self, event_loop: EventLoop<()>) {
        // * SETUP
        let mut start = Instant::now();
        let mut delta = DEFAULT_DELTA_TIME;
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.env.window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    event => {
                        input::input(&mut self, event);
                    }
                },
                Event::MainEventsCleared => {
                    // * UPDATE LORENZ
                    if !self.lorenz_state.paused {
                        self.update_lorenz(DEFAULT_DELTA_TIME)
                    }
                    // * UPDATE CAMERA
                    if self.env.cursor_grab {
                        self.camera.update(delta);
                        self.update_camera_buffer();
                    }
                    // * RENDER
                    self.render_state
                        .render_call(&self.env, &self.camera.bind_group);
                }
                Event::RedrawEventsCleared => {
                    delta = start.elapsed().as_secs_f32();
                    println!("{}", 1. / delta);
                    start = Instant::now();
                }

                Event::DeviceEvent {
                    device_id: _,
                    event,
                } => {
                    self.camera.controller.handle_mouse_movement(&event);
                }
                _ => {}
            }
        })
    }

    fn update_camera_buffer(&self) {
        self.env.queue.write_buffer(
            &self.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.camera.uniform.view_proj]),
        );
    }
    pub fn update_lorenz(&mut self, dt: f32) {
        self.compute_state.compute_call(&self.env);
        // self.lorenz_state.update(dt);
        // self.render_state
        //     .instances
        //     .update(&self.lorenz_state, &self.env.queue)
    }
}
