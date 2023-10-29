use bevy_ecs::{system::Resource, component::Component};
use glam::Vec2;

use crate::{material::Material, prelude::{Vertex, Instance}, primitives::rect, state::State};

#[derive(Resource)]
pub struct AssetServer<T: Component> {
    pub material_assets: Vec<Material>,
    pub meshes_to_be_loaded: Vec<(Vec<Vertex>, Vec<u32>, Vec<(Instance,T)>, usize, bool)>, //vertices, indices, instances, material_index, is_updating
    pub materials_to_be_loaded: Vec<String>,
    pub material_index: usize,
}
impl<T: Component + Clone> AssetServer<T> {
    pub fn new() -> Self {
        Self {
            material_assets: vec![],
            meshes_to_be_loaded: vec![],
            materials_to_be_loaded: vec![],
            material_index: 0
        }
    }
    pub fn queue_material(&mut self, material_path: &str) -> usize {
        self.materials_to_be_loaded.push(material_path.to_string());
        self.material_index += 1;
        return self.material_index - 1;
    }
    pub fn queue_mesh(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>, instances: Vec<(Instance,T)>, material_index: usize, is_updating: bool) {
        self.meshes_to_be_loaded.push((vertices,indices,instances,material_index,is_updating));
    }
    pub fn queue_sprites(&mut self, instances: Vec<(Instance,T)>, material_index: usize, is_updating: bool) {
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
}