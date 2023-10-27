use glam::{Mat4, Vec3};
use instant::Duration;
use wgpu::util::DeviceExt;
use wgpu::{Device, SurfaceConfiguration, Buffer, BindGroupLayout, BindGroup};

use crate::structs::CameraController;
use crate::state::State;



#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pos: [f32; 4],
}
impl CameraUniform {
    pub fn new() -> Self {
        Self {
            pos: [0.0; 4],
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.pos = camera.position.extend(1.0).into();
        println!("{:#?}", self.pos);
    }
}
pub struct CameraStruct{
    pub camera_uniform: CameraUniform,
    pub buffer: Buffer,
    pub bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup,
    pub camera_transform: Camera,
    pub camera_controller: CameraController
}
impl CameraStruct{
    pub fn new(device: &Device, config: &SurfaceConfiguration, camera: Camera, camera_controller: CameraController) -> Self{
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);
    
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
    
        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });
    
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });
        Self {camera_uniform, buffer, bind_group_layout, bind_group, camera_transform: camera, camera_controller }
    }
}


#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,
}

impl Camera {
    pub fn new(
        position: Vec3,
    ) -> Self {
        Self {
            position: position.into(),
        }
    }
}

pub fn default_cam(state: &mut State, dt: Duration) {
    let dt = dt.as_secs_f32();
    let mut camera = &mut state.camera.camera_transform;
    let controller = &mut state.camera.camera_controller;
    // Move left/right and up/down
    camera.position.x += (controller.amount_right - controller.amount_left) * controller.speed * dt;
    camera.position.y += (controller.amount_up - controller.amount_down) * controller.speed * dt;
}
