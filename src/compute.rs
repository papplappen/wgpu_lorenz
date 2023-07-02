use std::borrow::Cow;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, Buffer, BufferUsages, CommandEncoderDescriptor, ComputePassDescriptor,
    ComputePipeline, ComputePipelineDescriptor, Device, PipelineLayoutDescriptor, Queue,
    ShaderModuleDescriptor, ShaderSource, ShaderStages,
};

use crate::{
    config::{Config, ConfigComputeShader},
    env::Environment,
};

pub struct ComputeState {
    compute_pipeline: ComputePipeline,
    bind_group: BindGroup,
    delta_time_buffer: Buffer,
}

impl ComputeState {
    pub fn new(
        device: &Device,
        instance_buffer: &Buffer,
        config: &Config,
        delta_time: f32,
    ) -> Self {
        let delta_time_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Delta Time Buffer"),
            contents: &delta_time.to_ne_bytes(),
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        });
        let (bind_group_layout, bind_group) =
            Self::create_bind_group(device, instance_buffer, config, &delta_time_buffer);
        let compute_pipeline = Self::create_compute_pipeline(device, &[&bind_group_layout]);

        Self {
            compute_pipeline,
            bind_group,
            delta_time_buffer,
        }
    }

    fn create_compute_pipeline(
        device: &Device,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> ComputePipeline {
        let compute_wgsl = include_str!("compute.wgsl");

        let compute_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: ShaderSource::Wgsl(Cow::from(compute_wgsl)),
        });

        let compute_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Compute Shader Pipeline Layout"),
            bind_group_layouts,
            push_constant_ranges: &[],
        });

        device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Compute Shader Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "cs_main",
        })
    }

    fn create_bind_group(
        device: &Device,
        instance_buffer: &Buffer,
        config: &Config,
        delta_buffer: &Buffer,
    ) -> (BindGroupLayout, BindGroup) {
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Compute Bind Group Layout"),
            entries: &[
                // *INSTANCE BUFFER
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
                // * CONFIG
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
                // * DELTA TIME
                BindGroupLayoutEntry {
                    binding: 2,
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
            label: Some("Compute Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                // * INSTANCE_BUFFER
                BindGroupEntry {
                    binding: 0,
                    resource: instance_buffer.as_entire_binding(),
                },
                // * CONFIG
                BindGroupEntry {
                    binding: 1,
                    resource: ConfigComputeShader::from(config)
                        .as_buffer(device)
                        .as_entire_binding(),
                },
                // * DELTA TIME
                BindGroupEntry {
                    binding: 2,
                    resource: delta_buffer.as_entire_binding(),
                },
            ],
        });
        (bind_group_layout, bind_group)
    }

    pub fn compute_call(&self, env: &Environment, num_workgroups: (u32, u32, u32)) {
        let mut encoder = env
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());
        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            compute_pass.dispatch_workgroups(num_workgroups.0, num_workgroups.1, num_workgroups.2);
        }

        env.queue.submit(Some(encoder.finish()));
    }
    pub fn update_delta_time_buffer(&self, delta_time: f32, queue: &Queue) {
        queue.write_buffer(&self.delta_time_buffer, 0, &delta_time.to_ne_bytes())
    }
}
