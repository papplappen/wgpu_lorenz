use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

use crate::{lorenz::DEFAULT_DELTA_TIME, state::State};

pub fn input(state: &mut State, event: &WindowEvent) -> bool {
    // * HANDLE CAMERA INPUT FIRST
    if state.camera.controller.handle_key_input(event) {
        true
    } else {
        match event {
            // * STEP WHEN PAUSED
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::Return)
                    && input.state == ElementState::Released
                    && state.lorenz_state.paused =>
            {
                state.update_lorenz(DEFAULT_DELTA_TIME);
                true
            }
            // * TOGGLE PAUSE
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::Space)
                    && input.state == ElementState::Released =>
            {
                state.lorenz_state.paused = !state.lorenz_state.paused;
                true
            }
            // * TOGGLE CURSOR GRAB
            WindowEvent::KeyboardInput { input, .. }
                if input.virtual_keycode == Some(VirtualKeyCode::Slash)
                    && input.state == ElementState::Released =>
            {
                println!("{}",state.camera.entity.pos);
                println!("{}",state.camera.entity.dir);
                
                if state.env.cursor_grab {
                    state
                        .env
                        .window
                        .set_cursor_grab(winit::window::CursorGrabMode::None)
                        .unwrap();
                    state.env.window.set_cursor_visible(true);
                } else {
                    state
                        .env
                        .window
                        .set_cursor_grab(winit::window::CursorGrabMode::Confined)
                        .unwrap();
                    state.env.window.set_cursor_visible(false);
                }
                state.env.cursor_grab = !state.env.cursor_grab;
                true
            }
            _ => false,
        }
    }
}
