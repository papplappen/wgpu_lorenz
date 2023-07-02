use std::time::Instant;

use winit::event_loop::EventLoop;

use crate::{
    camera::Camera, compute::ComputeState, config::Config, env::Environment, input,
    lorenz::LorenzState, render::RenderState,
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
    pub config: Config,
    pub delta_time: f32,
    pub paused: bool,
}

impl State {
    pub fn run(mut self, event_loop: EventLoop<()>) {
        // * SETUP
        let mut start = Instant::now();
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
                    if !self.paused {
                        self.update_lorenz()
                    }
                    // * UPDATE CAMERA
                    if self.env.cursor_grab {
                        self.camera.update(self.delta_time, &self.env.queue);
                    }
                    // * RENDER
                    self.render_state.render_call(
                        &self.env,
                        &self.camera.bind_group,
                        self.config.num_lorenz_points,
                    );
                }
                Event::RedrawEventsCleared => {
                    self.delta_time = start.elapsed().as_secs_f32();
                    self.compute_state
                        .update_delta_time_buffer(self.delta_time, &self.env.queue);
                    println!("{}", 1. / self.delta_time);
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

    pub fn update_lorenz(&mut self) {
        self.compute_state
            .compute_call(&self.env, self.config.num_workgroups);
        // self.lorenz_state.update(dt);
        // self.render_state
        //     .instances
        //     .update(&self.lorenz_state, &self.env.queue)
    }
}
