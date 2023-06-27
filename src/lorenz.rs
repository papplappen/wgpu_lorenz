use glam::Vec3;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

use crate::compute::NUM_WORKGROUPS_PER_DIM;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
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

pub const NUMBER_LORENZ_POINTS: usize =
    (NUM_WORKGROUPS_PER_DIM * NUM_WORKGROUPS_PER_DIM * NUM_WORKGROUPS_PER_DIM) as usize;
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
        state + dt * self.delta(state)
    }
    pub fn as_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(self),
            usage: BufferUsages::UNIFORM,
        })
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
                x: -10. + 20. * (i as f32) / (NUMBER_LORENZ_POINTS as f32),
                y: 0.,
                z: -10. + 20. * (i as f32) / (NUMBER_LORENZ_POINTS as f32),
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
            .for_each(|p| *p = self.lorenz_config.step(dt, *p));
    }
}
