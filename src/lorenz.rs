use glam::DVec3;

pub struct LorenzConfig {
    pub rho: f64,
    pub sigma: f64,
    pub beta: f64,
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

pub const NUMBER_LORENZ_POINTS: usize = 1000;
pub const DEFAULT_DELTA_TIME: f64 = 0.0001;
impl LorenzConfig {
    fn delta(&self, state: DVec3) -> DVec3 {
        let DVec3 { x, y, z } = state;
        DVec3 {
            x: self.sigma * (y - x),
            y: x * (self.rho - z) - y,
            z: x * y - self.beta * z,
        }
    }

    pub fn step(&self, dt: f64, state: DVec3) -> DVec3 {
        dt * self.delta(state)
    }
}

pub struct LorenzState {
    pub lorenz_config: LorenzConfig,
    pub points: [DVec3; NUMBER_LORENZ_POINTS],
    pub paused: bool,
}
impl LorenzState {
    pub fn new(lorenz_config: LorenzConfig) -> Self {
        let mut i = 0.;
        let points = [0; NUMBER_LORENZ_POINTS].map(|_| {
            i += 0.5;
            DVec3 { x: i, y: 0., z: 0. }
        });

        Self {
            lorenz_config,
            points,
            paused: true,
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.points = self
            .points
            .map(|pos| pos + self.lorenz_config.step(dt, pos));
    }
}
