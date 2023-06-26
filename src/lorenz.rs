use glam::Vec3;

pub struct LorenzConfig {
    pub rho: f32,
    pub sigma: f32,
    pub beta: f32,
}

impl Default for LorenzConfig {
    fn default() -> Self {
        Self {
            rho: 28.,
            sigma: 10.,
            beta: 8. / 3.,
        }
    }
}

pub const NUMBER_LORENZ_POINTS: usize = 1000000;
pub const DEFAULT_DELTA_TIME: f32 = 0.01;
impl LorenzConfig {
    fn delta(&self, state: Vec3) -> Vec3 {
        let Vec3 { x, y, z } = state;
        Vec3 {
            x: self.sigma * (y - x),
            y: x * (self.rho - z) - y,
            z: x * y - self.beta * z,
        }
    }

    pub fn step(&self, dt: f32, state: Vec3) -> Vec3 {
        dt * self.delta(state)
    }
}

pub struct LorenzState {
    pub lorenz_config: LorenzConfig,
    pub points: Vec<Vec3>,
    pub paused: bool,
}
impl LorenzState {
    pub fn new(lorenz_config: LorenzConfig) -> Self {
        let points = (0..NUMBER_LORENZ_POINTS)
            .map(|i| Vec3 {
                x: (i as f32) / (NUMBER_LORENZ_POINTS as f32),
                y: 0.,
                z: 0.,
            })
            .collect();
        Self {
            lorenz_config,
            points,
            paused: true,
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.points
            .iter_mut()
            .for_each(|p| *p += self.lorenz_config.step(dt, *p));
    }
}
