use glam::{Vec2, Vec3};
use image::RgbImage;
use tile_based_game::{material::Material, prelude::Instance, primitives::rect, state::State};

pub fn gen(state: &mut State, width: usize, height: usize) -> World {
    let mut world = World::new(width, height);
    dirt_pass(&mut world);
    world
}
fn dirt_pass(world: &mut World) {}
async fn render_chunk(chunk: &mut Chunk, state: &mut State) {
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
    let (vertices, indices) = rect(p1, p2);
    state.build_mesh(
        vertices,
        indices,
        instances.iter_mut().map(|instance| instance).collect(),
        state.compile_material("cube-diffuse.jpg").await,
        false,
    );
}
#[derive(Clone, Copy)]
pub struct Block {
    pub block_id: u32,
}
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
        };
        World {
            world_width: width,
            world_height: height,
            block_ids_list: vec![],
            chunks: vec![vec![chunks; width]; height],
            chest_locations: vec![],
        }
    }
    pub fn get_block(&mut self, row: usize, col: usize) -> Option<&mut Block> {
        if row > self.world_width || row < 0 || col > self.world_height || col < 0 {
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
    pub fn save_to_image(&self, image_loc: &str) {
        let mut image = RgbImage::new(self.world_width as u32, self.world_height as u32);
        *image.get_pixel_mut(5, 5) = image::Rgb([255, 255, 255]);
        image.save(image_loc).unwrap();
    }
}
#[derive(Clone, Copy)]
pub struct Chunk {
    pub rows: [[Block; 16]; 16], //16x16 squares
}
pub struct Chest {}
pub struct BlockType {
    pub mining_power: u16,
    pub block_sprite: Material,
}
