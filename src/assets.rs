use bevy_ecs::system::Resource;
use glam::Vec2;

use crate::{material::Material, prelude::{Vertex, Instance}, primitives::rect};

#[derive(Resource)]
pub struct AssetServer {
    pub material_assets: Vec<Material>,
    pub meshes_to_be_loaded: Vec<(Vec<Vertex>, Vec<u32>, Vec<Instance>, usize, bool)>, //vertices, indices, instances, material_index, is_updating
    pub materials_to_be_loaded: Vec<String>,
    pub material_index: usize,
}
impl AssetServer {
    pub fn queue_material(&mut self, material_path: &str) -> usize {
        self.materials_to_be_loaded.push(material_path.to_string());
        self.material_index += 1;
        return self.material_index - 1;
    }
    pub fn queue_mesh(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>, instances: Vec<Instance>, material_index: usize, is_updating: bool) {
        self.meshes_to_be_loaded.push((vertices,indices,instances,material_index,is_updating));
    }
    pub fn queue_sprites(&mut self, instances: Vec<Instance>, material_index: usize, is_updating: bool) {
        let (vertices, indices) = rect(Vec2::new(0.5,0.5), Vec2::new(-0.5,-0.5));
        self.meshes_to_be_loaded.push((vertices,indices,instances,material_index,is_updating));
    }
}