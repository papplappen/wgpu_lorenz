use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

use crate::lorenz::LorenzConfig;

pub const DEFAULT_DELTA_TIME: f32 = 0.01;

const NUMBER_LORENZ_POINTS: usize = 1000000;

const SMOOTH_SHADING: bool = false;

pub struct Config {
    pub lorenz: LorenzConfig,
    pub num_lorenz_points: usize,
    pub num_workgroups: (u32, u32, u32),
    pub smooth_shading: bool,
}

impl Default for Config {
    fn default() -> Self {
        let temp_num_workgroups = (NUMBER_LORENZ_POINTS as f64).powf(1. / 3.) as u32;
        Self {
            lorenz: LorenzConfig::default(),
            num_lorenz_points: (temp_num_workgroups as usize).pow(3),
            num_workgroups: (
                temp_num_workgroups,
                temp_num_workgroups,
                temp_num_workgroups,
            ),
            smooth_shading: SMOOTH_SHADING,
        }
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct ConfigComputeShader {
    lorenz: LorenzConfig,
    num_workgroups: [u32; 3],
    _pad1: f32,
}
impl From<&Config> for ConfigComputeShader {
    fn from(cfg: &Config) -> Self {
        Self {
            lorenz: cfg.lorenz,
            num_workgroups: [
                cfg.num_workgroups.0,
                cfg.num_workgroups.1,
                cfg.num_workgroups.2,
            ],
            _pad1: f32::NAN,
        }
    }
}
impl ConfigComputeShader {
    pub fn as_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Config Buffer"),
            contents: bytemuck::bytes_of(self),
            usage: BufferUsages::UNIFORM,
        })
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct ConfigDrawShader {
    smooth_shading: u32,
}
impl From<&Config> for ConfigDrawShader {
    fn from(cfg: &Config) -> Self {
        Self {
            smooth_shading: cfg.smooth_shading as u32,
        }
    }
}
impl ConfigDrawShader {
    pub fn as_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Config Buffer"),
            contents: bytemuck::bytes_of(self),
            usage: BufferUsages::UNIFORM,
        })
    }
}
