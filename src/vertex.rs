use glam::Vec3;
use wgpu::{util::DeviceExt, Buffer, BufferUsages, Device, VertexBufferLayout};

pub const SQUARE: [Vec3; 4] = [
    Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    },
    Vec3 {
        x: 1.,
        y: 0.,
        z: 0.,
    },
    Vec3 {
        x: 0.,
        y: 1.,
        z: 0.,
    },
    Vec3 {
        x: 1.,
        y: 1.,
        z: 0.,
    },
];
const COLOR: [f32; 3] = [1., 0., 0.];
pub fn create_vertex_buffer(device: &Device) -> Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&SQUARE.map(|p| Vertex {
            position: p.into(),
            color: COLOR,
        })),
        usage: BufferUsages::VERTEX,
    })
}
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
    pub fn desc() -> VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
