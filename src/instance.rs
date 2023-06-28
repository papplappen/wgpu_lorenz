use glam::Vec3;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferAddress, BufferUsages, Color, Device, Queue,
    VertexBufferLayout, VertexStepMode,
};

use crate::lorenz::LorenzState;

#[derive(Clone, Copy)]
pub struct Instance {
    pub position: Vec3,
    pub color: Color,
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]

pub struct RawInstance {
    pos: [f32; 3],
    _pad1: f32,
    color: [f32; 3],
    _pad2: f32,
}
impl From<Instance> for RawInstance {
    fn from(instance: Instance) -> Self {
        Self {
            pos: instance.position.to_array(),
            color: [
                instance.color.r as f32,
                instance.color.g as f32,
                instance.color.b as f32,
            ],
            _pad1: 0.,
            _pad2: 0.,
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
    pub data: Vec<Instance>,
    pub raw: Vec<RawInstance>,
    pub buffer: Buffer,
}
// const DEFAULT_COLOR: Color = Color {
//     r: 219. / 256.,
//     g: 285. / 256.,
//     b: 6. / 256.,
//     a: 1.0,
// };

const DEFAULT_COLOR : &str = &"0000ff";

impl From<(&LorenzState, &Device)> for InstancesVec {
    fn from((lorenz_state, device): (&LorenzState, &wgpu::Device)) -> Self {
        let instances: Vec<Instance> = lorenz_state
            .points
            .iter()
            .map(|pos| Instance {
                position: *pos,
                color: color_from_hex(DEFAULT_COLOR).unwrap(),
            })
            .collect();
        let raw: Vec<RawInstance> =
            instances.iter().map(|i| (*i).into()).collect();

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&raw),
            usage: BufferUsages::VERTEX
                | BufferUsages::COPY_DST
                | BufferUsages::STORAGE,
        });
        Self {
            data: instances,
            raw,
            buffer,
        }
    }
}
impl InstancesVec {
    pub fn update(&mut self, lorenz_state: &LorenzState, queue: &Queue) {
        self.data = lorenz_state
            .points
            .iter()
            .map(|pos| Instance {
                position: *pos,
                color: color_from_hex(DEFAULT_COLOR).unwrap(),
            })
            .collect();
        self.raw = self.data.iter().map(|i| (*i).into()).collect();
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.raw));
    }
}

/// For example:
/// ```rs
/// let color = color_from_hex("dbafea").unwrap();
/// ```
fn color_from_hex(hex: &str) -> Result<Color, std::num::ParseIntError> {
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;
    Ok(Color {
        r: (r as f64) / 256.,
        g: (g as f64) / 256.,
        b: (b as f64) / 256.,
        a: 1.,
    })
}
