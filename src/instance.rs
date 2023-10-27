use bevy_ecs::component::Component;
use glam::{Quat, Vec3, Mat4, Vec2};

use crate::resources::UpdateInstance;

#[derive(Debug, Copy, Clone, Component)]
pub struct Instance {
    pub position: Vec3,
    pub is_world_space: bool,
    pub prefab_index: usize,
    pub enabled: bool
}
impl Default for Instance {
    fn default() -> Self {
        Instance { position: Vec3::ZERO, is_world_space: true, prefab_index: 0, enabled: true }
    }
}

impl Instance {
    pub fn to_raw(&self) -> Option<InstanceRaw> {
        if self.enabled {Some(InstanceRaw::new(self.position, self.is_world_space))} else {None}
    }
    pub fn update(&self, instances: Vec<InstanceRaw>, instance_update: &mut UpdateInstance) {
        instance_update.prefab_slab.get_mut(self.prefab_index).unwrap().update_buffer(instances, &instance_update.queue);
    }
    pub fn pos_2d(&self) -> Vec2 {
        Vec2::new(self.position.x, self.position.y)
    }
    pub fn pos(&self) -> Vec3 {
        self.position
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
    is_world_space: u32,
}

impl InstanceRaw {
    pub fn new(position: Vec3, is_world_space: bool) -> Self {
        Self {
            model: Mat4::from_translation(position).to_cols_array_2d(),
            is_world_space: if is_world_space { 0 } else { 1 },
        }
    }
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}