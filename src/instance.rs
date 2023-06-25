use glam::DVec3;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferAddress, BufferUsages, Device, VertexBufferLayout, VertexStepMode,
};

use crate::lorenz::{LorenzState, NUMBER_LORENZ_POINTS};

#[derive(Clone, Copy)]
pub struct Instance {
    pub position: DVec3,
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawInstance {
    pos: [f64; 3],
}
impl From<Instance> for RawInstance {
    fn from(instance: Instance) -> Self {
        Self {
            pos: instance.position.to_array(),
        }
    }
}
impl RawInstance {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![1 => Float32x3, 2 => Float32x3];
    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<RawInstance>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
pub struct InstancesVec {
    pub data: [Instance; NUMBER_LORENZ_POINTS],
    pub raw: [RawInstance; NUMBER_LORENZ_POINTS],
    pub buffer: Buffer,
}

impl From<(&LorenzState, &Device)> for InstancesVec {
    fn from((lorenz_state, device): (&LorenzState, &wgpu::Device)) -> Self {
        let instances = lorenz_state.points.map(|pos| Instance { position: pos });
        let raw = instances.map(|i| i.into());

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&raw),
            usage: BufferUsages::VERTEX | BufferUsages::STORAGE,
        });
        Self {
            data: instances,
            raw,
            buffer,
        }
    }
}
