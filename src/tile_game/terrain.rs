use bevy_ecs::system::{Query, Resource, Res, ResMut, Commands};
use glam::{Vec2, Vec3};
use image::RgbImage;
use noise::{Perlin, permutationtable::PermutationTable};
use tile_based_game::{material::Material, prelude::Instance, primitives::rect, state::State};

use super::{terrain_passes::basic_caves_pass, player::Player};
pub fn gen(width: usize, height: usize, seed: u32) -> World {
    let mut world = World::new(width, height);
    let hasher = PermutationTable::new(seed);
    basic_caves_pass(&mut world, &hasher);
    world
}
pub fn chunk_render_checker(mut commands: Commands, terrain_world: ResMut<World>, player: Res<Player>) {
    let mut col_idx: u32 = 0;
    let (player_x, player_y) = player.block_position();
    for chunk_col in &mut terrain_world.chunks { //TODO: Change this to be only chunks near the player for efficiency
        let mut row_idx: u32 = 0;
        for chunk in chunk_col {
            let chunk_x = col_idx * 16;
            let chunk_y = col_idx * 16;
            let x_dif = chunk_x.abs_diff(player_x);
            let y_dif = chunk_y.abs_diff(player_y);
            if chunk.rendered {
                
            }
            else{
                if x_dif < 17 && y_dif < 17 {
                    chunk.rendered = true;
                    render_chunk(chunk, state, commands).await //SEPERATE WORLD AND SCHEDULE FROM STATE THEN MAKE STATE A RESOURCE SO YOU CAN BUILD MESHES IN RUNTIME
                }
            }
            row_idx += 1;
        }
        col_idx += 1;
    }
}
async fn render_chunk(chunk: &mut Chunk, state: &mut State, commands: &mut Commands) {
    let mut row_idx = 0;
    let mut instances = vec![];
    let block_size = 0.2;
    let rows = &chunk.rows;
    for col in rows {
        let mut col_idx = 0;
        for block in col {
            let instance = Instance {
                position: Vec3::new(col_idx as f32 * block_size, row_idx as f32 * block_size, 0.), //change to swithc pos
                ..Default::default()
            };
            instances.push(instance);
            col_idx += 1;
        }
        row_idx += 1;
    }
    let block_size_halfed = block_size / 2.;
    let p1 = Vec2::new(-block_size_halfed, -block_size_halfed);
    let p2 = Vec2::new(block_size_halfed, block_size_halfed);
    let material = state.compile_material("cube-diffuse.jpg", &mut world).await;
    let (vertices, indices) = rect(p1, p2);
    state.build_mesh(
        vertices,
        indices,
        instances.iter_mut().map(|instance| instance).collect(),
        state.compile_material("cube-diffuse.jpg", &mut world).await,
        false, &mut world
    );
}
#[derive(Clone, Copy)]
pub struct Block {
    pub block_id: u32,
}
#[derive(Resource)]
pub struct World {
    pub world_width: usize,
    pub world_height: usize,
    pub block_ids_list: Vec<BlockType>,
    pub chunks: Vec<Vec<Chunk>>, //each row contains columns
    pub chest_locations: Vec<Chest>,
}
impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let chunks = Chunk {
            rows: [[Block { block_id: 0 }; 16]; 16],
            rendered: false,
        };
        World {
            world_width: width / 16 * 16,
            world_height: height / 16 * 16, // to round to the nearest 16, not unnecessary
            block_ids_list: vec![],
            chunks: vec![vec![chunks; width / 16]; height / 16],
            chest_locations: vec![],
        }
    }
    pub fn get_block(&mut self, row: usize, col: usize) -> Option<&mut Block> {
        if row > self.world_height || col > self.world_width {
            return None;
        }
        let chunk_idx_row = row / 16;
        let chunk_idx_col = col / 16;
        let block_idx_row = row % 16;
        let block_idx_col = col % 16;
        return Some(
            &mut self.chunks[chunk_idx_row][chunk_idx_col].rows[block_idx_row][block_idx_col],
        );
    }
    pub fn save_to_image(&mut self, image_loc: &str) {
        let mut image = RgbImage::new(self.world_width as u32, self.world_height as u32);
        for row in 0..self.world_height {
            for col in 0..self.world_width {
                let block_id = self.get_block(row, col).unwrap().block_id;
                let rgb = if block_id == 0 {
                    [30, 125, 214] //air
                } else if block_id == 1 {
                    [118, 85, 43] //dirt
                } else if block_id == 2 {
                    [120, 115, 102] //stone
                }else{
                    [183,176,156]
                };
                *image.get_pixel_mut(col as u32, row as u32) = image::Rgb(rgb);
            }
        }
        image.save(image_loc).unwrap();
    }
}
#[derive(Clone, Copy)]
pub struct Chunk {
    pub rows: [[Block; 16]; 16], //16x16 squares
    pub rendered: bool,
}
pub struct Chest {}
pub struct BlockType {
    pub mining_power: u16,
    pub block_sprite: Material,
}
