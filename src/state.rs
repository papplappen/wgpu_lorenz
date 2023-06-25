use winit::event_loop::EventLoop;

use crate::{camera::Camera, env::Environment, lorenz::LorenzState, render::RenderState};
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};
pub struct State {
    pub env: Environment,
    pub render_state: RenderState,
    pub lorenz_state: LorenzState,
    pub camera: Camera,
}
impl State {
    pub fn run(mut self, event_loop: EventLoop<()>) {
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

                    we => {
                        self.camera.controller.handle_key_input(we);
                        // dbg!(event);
                    }
                },
                Event::MainEventsCleared => {
                    // Update stuff
                    self.lorenz_state.update(0.1);
                    self.camera.update(0.1);
                    self.render_state
                        .render_call(&self.env, &self.camera.bind_group);
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
}
