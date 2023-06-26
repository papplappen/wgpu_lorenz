use wgpu::{
    include_wgsl, BindGroup, BindGroupLayout, BlendState, Buffer, Color, ColorTargetState,
    ColorWrites, CommandEncoderDescriptor, DepthBiasState, DepthStencilState, Device, Extent3d,
    FragmentState, MultisampleState, Operations, PipelineLayoutDescriptor, PrimitiveState,
    PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, StencilState,
    SurfaceConfiguration, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor, VertexState,
};

use crate::{
    env::Environment,
    instance::{InstancesVec, RawInstance},
    lorenz::{LorenzState, NUMBER_LORENZ_POINTS},
    vertex::{Vertex, SQUARE},
};

const BACKGROUND_COLOR: Color = Color {
    r: 0.1,
    g: 0.2,
    b: 0.3,
    a: 1.0,
};
pub struct RenderState {
    pub vertex_buffer: Buffer,
    pub instances: InstancesVec,
    pub render_pipeline: RenderPipeline,
    pub depth_texture: TextureView,
}
impl RenderState {
    pub fn new(
        lorenz_state: &LorenzState,
        env: &Environment,
        camera_bind_group_layout: BindGroupLayout,
    ) -> Self {
        // * CREATE DEPTH TEXTURE
        let depth_texture = Self::create_depth_texture(&env.device, &env.config);

        // * CREATE VERTEX & INSTANCE BUFFERS
        let vertex_buffer = Vertex::create_vertex_buffer(&env.device);
        let instances = InstancesVec::from((lorenz_state, &env.device));

        // * CREATES RENDER PIPELINE
        let render_pipeline =
            Self::create_render_pipeline(&env.device, &env.config, &[&camera_bind_group_layout]);
        Self {
            vertex_buffer,
            render_pipeline,
            depth_texture,
            instances,
        }
    }

    pub fn render_call(&self, env: &Environment, camera_bind_group: &BindGroup) {
        let output = env.surface.get_current_texture().unwrap();
        let mut encoder = env
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Encoder"),
            });
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BACKGROUND_COLOR),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture,
                    depth_ops: Some(Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(0, camera_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            render_pass.set_vertex_buffer(1, self.instances.buffer.slice(..));

            render_pass.draw(0..SQUARE.len() as u32, 0..NUMBER_LORENZ_POINTS as u32)
        }
        env.queue.submit(Some(encoder.finish()));
        output.present();
    }

    fn create_render_pipeline(
        device: &Device,
        config: &SurfaceConfiguration,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> RenderPipeline {
        // * LOAD SHADER
        let draw_shader = device.create_shader_module(include_wgsl!("draw.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts,
            push_constant_ranges: &[],
        });
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &draw_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), RawInstance::desc()],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
                module: &draw_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview: None,
        })
    }
    fn create_depth_texture(device: &Device, config: &SurfaceConfiguration) -> TextureView {
        let size = Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        texture.create_view(&TextureViewDescriptor::default())
    }
}
