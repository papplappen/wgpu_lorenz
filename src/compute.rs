use wgpu::{
    include_wgsl, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, Buffer, CommandEncoderDescriptor,
    ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, Device,
    PipelineLayoutDescriptor, ShaderStages,
};

use crate::{env::Environment, lorenz::LorenzConfig};

pub struct ComputeState {
    compute_pipeline: ComputePipeline,
    bind_group: BindGroup,
}
const NUM_WORKGROUPS_PER_DIM: u32 = 100;

impl ComputeState {
    pub fn new(device: &Device, instance_buffer: &Buffer, lorenz_config: LorenzConfig) -> Self {
        let (bind_group_layout, bind_group) =
            Self::create_bind_group(device, instance_buffer, lorenz_config);
        let compute_pipeline = Self::create_compute_pipeline(device, &[&bind_group_layout]);

        Self {
            compute_pipeline,
            bind_group,
        }
    }

    fn create_compute_pipeline(
        device: &Device,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> ComputePipeline {
        let compute_shader = device.create_shader_module(include_wgsl!("compute.wgsl"));

        let compute_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts,
            push_constant_ranges: &[],
        });
        device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: None,
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "cs_main",
        })
    }

    fn create_bind_group(
        device: &Device,
        instance_buffer: &Buffer,
        lorenz_config: LorenzConfig,
    ) -> (BindGroupLayout, BindGroup) {
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        instance_buffer.as_entire_buffer_binding(),
                    ),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        lorenz_config.as_buffer(device).as_entire_buffer_binding(),
                    ),
                },
            ],
        });
        (bind_group_layout, bind_group)
    }

    pub fn compute_call(&self, env: &Environment) {
        let mut encoder = env
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());
        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            compute_pass.dispatch_workgroups(
                NUM_WORKGROUPS_PER_DIM,
                NUM_WORKGROUPS_PER_DIM,
                NUM_WORKGROUPS_PER_DIM,
            );
        }

        env.queue.submit(Some(encoder.finish()));
    }
}
