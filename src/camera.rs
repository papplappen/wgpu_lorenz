use glam::{Mat3, Mat4, Vec3};
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, Buffer,
    BufferUsages, Device, ShaderStages, SurfaceConfiguration,
};
use winit::event::{DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

const SPEED: f32 = 100.;
const SENS: f32 = 0.1;
pub struct Camera {
    pub entity: CameraEntity,
    pub uniform: CameraUniform,
    pub controller: CameraController,
    pub bind_group: BindGroup,
    pub buffer: Buffer,
}

impl Camera {
    pub fn create_camera(
        device: &Device,
        config: &SurfaceConfiguration,
    ) -> (Self, BindGroupLayout) {
        let entity = CameraEntity {
            pos: Vec3::ONE * 50.,
            dir: Vec3::NEG_ONE.normalize(),
            up: Vec3::Y,
            aspect_ratio: config.width as f32 / config.height as f32,
            fov_y: 45.0,
            z_near: 1.,
            z_far: 1000.0,
        };
        let mut uniform = CameraUniform::new();
        uniform.update(&entity);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Groups"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        let controller = CameraController::new(SPEED, SENS);

        (
            Self {
                entity,
                uniform,
                controller,
                bind_group,
                buffer,
            },
            bind_group_layout,
        )
    }

    pub fn update(&mut self, delta: f32) {
        self.controller
            .update_camera_entity(&mut self.entity, delta);
        self.uniform.update(&self.entity);
    }
}
#[derive(Debug)]
pub struct CameraEntity {
    pub pos: Vec3,
    pub dir: Vec3,
    pub up: Vec3,
    pub aspect_ratio: f32,
    pub fov_y: f32, // ! DEGREES
    pub z_near: f32,
    pub z_far: f32,
}

impl CameraEntity {
    pub fn build_view_projection_matrix(&self) -> CameraUniform {
        let view = Mat4::look_to_rh(self.pos, self.dir, self.up);
        let proj = Mat4::perspective_rh(
            self.fov_y.to_radians(),
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        );

        CameraUniform {
            view_proj: (proj * view).to_cols_array_2d(),
            proj: proj.to_cols_array_2d(),
        }
    }
}
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
    pub fn update(&mut self, camera_entity: &CameraEntity) {
        *self = camera_entity.build_view_projection_matrix();
    }
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self::new()
    }
}
pub struct CameraController {
    speed: f32,
    sens: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    delta: (f32, f32),
}

impl CameraController {
    pub fn new(speed: f32, sens: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            delta: (0., 0.),
            sens,
        }
    }

    pub fn handle_key_input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub fn handle_mouse_movement(&mut self, event: &DeviceEvent) -> bool {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.delta = (delta.0 as f32, delta.1 as f32);
            true
        } else {
            false
        }
    }
    pub fn update_camera_entity(&mut self, camera_entity: &mut CameraEntity, dt: f32) {
        camera_entity.dir = camera_entity.dir.normalize();
        let yaw = Mat3::from_rotation_y(-self.delta.0.to_radians() * self.sens);
        camera_entity.dir = yaw * camera_entity.dir;

        let pitch = Mat3::from_axis_angle(
            camera_entity.dir.cross(camera_entity.up).normalize(),
            -self.delta.1.to_radians() * self.sens,
        );
        self.delta = (0., 0.);
        let new_dir = pitch * camera_entity.dir;
        if camera_entity
            .dir
            .cross(camera_entity.up)
            .dot(new_dir.cross(camera_entity.up))
            >= 0.
        {
            camera_entity.dir = new_dir;
        } else {
            camera_entity.dir.y = camera_entity.dir.y.signum();
        }
        camera_entity.dir = camera_entity.dir.normalize();
        let forward = camera_entity.dir * self.speed * dt;
        if self.is_forward_pressed {
            camera_entity.pos += forward;
        }
        if self.is_backward_pressed {
            camera_entity.pos -= forward;
        }

        let right = camera_entity.dir.cross(camera_entity.up).normalize() * self.speed * dt;

        if self.is_right_pressed {
            camera_entity.pos += right;
        }
        if self.is_left_pressed {
            camera_entity.pos -= right;
        }
    }
}
