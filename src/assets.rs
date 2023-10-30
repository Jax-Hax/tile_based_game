use bevy_ecs::{system::Resource, component::Component, world::World};
use glam::Vec2;
use slab::Slab;
use wgpu::{Device, Queue, util::DeviceExt};

use crate::{material::Material, prelude::{Vertex, Instance}, primitives::rect, state::State, prefabs::Prefab, structs::Mesh, loader::load_texture};

#[derive(Resource)]
pub struct AssetServer {
    pub material_assets: Vec<Material>,
    pub meshes_to_be_loaded: Vec<(Vec<Vertex>, Vec<u32>, Vec<(Instance,)>, usize, bool)>, //vertices, indices, instances, material_index, is_updating
    pub materials_to_be_loaded: Vec<String>,
    pub material_index: usize,
    pub device: Device,
    pub queue: Queue,
    pub prefab_slab: Slab<Prefab>,
    pub build_path: String
}
impl AssetServer {
    pub fn new(device: Device, queue: Queue, build_path: String) -> Self {
        Self {
            material_assets: vec![],
            meshes_to_be_loaded: vec![],
            materials_to_be_loaded: vec![],
            material_index: 0,
            device,
            queue,
            prefab_slab: Slab::new(),
            build_path
        }
    }
    pub fn queue_material(&mut self, material_path: &str) -> usize {
        self.materials_to_be_loaded.push(material_path.to_string());
        self.material_index += 1;
        return self.material_index - 1;
    }
    pub fn queue_mesh(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>, instances: Vec<(Instance,)>, material_index: usize, is_updating: bool) {
        self.meshes_to_be_loaded.push((vertices,indices,instances,material_index,is_updating));
    }
    pub fn queue_sprites(&mut self, instances: Vec<(Instance,)>, material_index: usize, is_updating: bool) {
        let (vertices, indices) = rect(Vec2::new(0.5,0.5), Vec2::new(-0.5,-0.5));
        self.meshes_to_be_loaded.push((vertices,indices,instances,material_index,is_updating));
    }
    pub async fn next_frame(&mut self, state: &mut State) {
        for material_path in &self.materials_to_be_loaded {
            self.material_assets.push(state.compile_material(&material_path).await);
        }
        self.materials_to_be_loaded = vec![];
        for (vertices, indices, instances, material_idx, is_updating) in self.meshes_to_be_loaded.iter_mut() {
            state.build_mesh_internal(vertices, indices, instances.to_vec(), material_idx, is_updating)
        }
        self.material_index = self.material_assets.len();
    }
    pub async fn compile_material(&self, texture_name: &str, world: &mut World) -> Material {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let diffuse_texture =
            load_texture(texture_name, &self.build_path, &asset_server.device, &asset_server.queue)
                .await
                .unwrap();
        let texture_bind_group = asset_server.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: None,
        });
        Material {
            bind_group: texture_bind_group,
        }
    }
    pub fn build_mesh(
        &mut self,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        instances: Vec<&mut Instance>,
        material_idx: usize,
        is_updating: bool,
        world: &mut World
    ) {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let mesh = Mesh {
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material_idx,
        };
        let mut instance_data = vec![];
        let mut length = 0;
        for instance in &instances {
            let instance_raw = instance.to_raw();
            if instance_raw.is_some() {
                instance_data.push(instance_raw.unwrap());
                length += 1;
            }
        }
        let instance_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: if is_updating {
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::VERTEX
                },
            });
        let container = Prefab::new(
            instance_buffer,
            mesh,
            length,
        );
        let mut update_instance = world.get_resource_mut::<UpdateInstance>().unwrap();
        let entry = update_instance.prefab_slab.vacant_entry();
        let key = entry.key();
        for instance in instances {
            instance.prefab_index = key;
        }
        entry.insert(container);
    }
    pub fn build_mesh_internal<T: Component>(
        &mut self,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        instances: Vec<(Instance,T)>,
        material_idx: &mut usize,
        is_updating: &mut bool,
    ) {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let mesh = Mesh {
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material_idx: *material_idx,
        };
        let mut instance_data = vec![];
        let mut length = 0;
        for (instance,_) in &instances {
            let instance_raw = instance.to_raw();
            if instance_raw.is_some() {
                instance_data.push(instance_raw.unwrap());
                length += 1;
            }
        }
        let instance_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: if *is_updating {
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::VERTEX
                },
            });
        let container = Prefab::new(
            instance_buffer,
            mesh,
            length,
        );
        let mut update_instance = self.world.get_resource_mut::<UpdateInstance>().unwrap();
        let entry = update_instance.prefab_slab.vacant_entry();
        let key = entry.key();
        for (mut instance,_) in instances {
            instance.prefab_index = key;
        }
        entry.insert(container);
        self.world.spawn_batch(instances);
    }
    pub fn make_sprites(
        &mut self,
        instances: Vec<&mut Instance>,
        material_idx: usize,
        is_updating: bool
    ) {
        //make sprite mesh
        let (vertices, indices) = rect(Vec2::new(0.5,0.5), Vec2::new(-0.5,-0.5));
        let vertex_buffer = self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });
            
        let mesh = Mesh {
            vertex_buffer,index_buffer, num_elements: indices.len() as u32,
            material_idx,
        };
        let mut instance_data = vec![];
        let mut length = 0;
        for instance in &instances {
            let instance_raw = instance.to_raw();
            if instance_raw.is_some() {
                instance_data.push(instance_raw.unwrap());
                length += 1;
            }
        }
        let instance_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: if is_updating {
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::VERTEX
                },
            });
        let container = Prefab::new(
            instance_buffer,
            mesh,
            length,
        );
        let mut update_instance = self.world.get_resource_mut::<UpdateInstance>().unwrap();
        let entry = update_instance.prefab_slab.vacant_entry();
        let key = entry.key();
        for instance in instances {
            instance.prefab_index = key;
        }
        entry.insert(container);
    }
}