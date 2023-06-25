use env::Environment;
use pollster::FutureExt;
use winit::event_loop::EventLoop;

mod env;

fn main() {
    let event_loop = EventLoop::new();
    let env = Environment::new(&event_loop).block_on();
    env.run(event_loop);
}
