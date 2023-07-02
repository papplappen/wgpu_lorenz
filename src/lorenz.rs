use glam::Vec3;
use rand::Rng;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LorenzConfig {
    pub rho: f32,
    pub sigma: f32,
    pub beta: f32,
    pub step_size_factor: f32,
}

impl Default for LorenzConfig {
    fn default() -> Self {
        Self {
            rho: 28.,
            sigma: 10.,
            beta: 8. / 3.,
            step_size_factor: 0.5,
        }
    }
}

impl LorenzConfig {
    fn _delta(&self, state: Vec3) -> Vec3 {
        let Vec3 { x, y, z } = state;
        Vec3 {
            x: self.sigma * (y - x),
            y: x * (self.rho - z) - y,
            z: x * y - self.beta * z,
        }
    }

    pub fn _step(&self, dt: f32, state: Vec3) -> Vec3 {
        state + self.step_size_factor * dt * self._delta(state)
    }
}

pub struct LorenzState {
    pub points: Vec<Vec3>,
}
impl LorenzState {
    pub fn new(number_lorenz_points: usize) -> Self {
        let points = (0..number_lorenz_points)
            .map(|_| {
                const N: f32 = 50.;
                Vec3 {
                    x: rand::thread_rng().gen_range(-N..N),
                    y: rand::thread_rng().gen_range(-N..N),
                    z: rand::thread_rng().gen_range(-N..N),
                }
            })
            .collect();
        Self { points }
    }
    pub fn _update(&mut self, dt: f32, lorenz_config: LorenzConfig) {
        self.points
            .iter_mut()
            .for_each(|p| *p = lorenz_config._step(dt, *p));
    }
}
