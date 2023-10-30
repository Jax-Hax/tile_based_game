use bevy_ecs::system::{Resource, Res, ResMut};
use glam::{Vec2, Vec3};
use image::RgbImage;
use noise::permutationtable::PermutationTable;
use tile_based_game::{material::Material, prelude::Instance, primitives::rect_with_tex_coords, assets::AssetServer};

use super::{terrain_passes::basic_caves_pass, player::Player};
pub fn gen(width: usize, height: usize, seed: u32, sprite_map_idx: usize) -> World {
    let mut world = World::new(width, height, sprite_map_idx);
    let hasher = PermutationTable::new(seed);
    basic_caves_pass(&mut world, &hasher);
    world
}
pub fn chunk_render_checker(mut world: ResMut<World>, player: Res<Player>, mut asset_server: ResMut<AssetServer>) {
    let mut col_idx: u32 = 0;
    let (player_x, player_y) = player.block_position();
    let sprite_sheet_idx = world.sprite_map_idx;
    for chunk_col in &mut world.chunks { //TODO: Change this to be only chunks near the player for efficiency
        let mut row_idx: u32 = 0;
        for chunk in chunk_col {
            let chunk_x = row_idx * 16;
            let chunk_y = col_idx * 16;
            let x_dif = chunk_x.abs_diff(player_x);
            let y_dif = chunk_y.abs_diff(player_y);
            if chunk.rendered {
                if x_dif > 33 && y_dif < 33 {
                    chunk.rendered = false;
                    //render_chunk(chunk, &mut asset_server, sprite_sheet_idx);
                }
            }
            else{
                if x_dif < 17 && y_dif < 17 {
                    chunk.rendered = true;
                    println!(" i did ");
                    render_chunk(chunk, &mut asset_server, sprite_sheet_idx, chunk_x, chunk_y);
                }
            }
            row_idx += 1;
        }
        col_idx += 1;
    }
}
fn render_chunk(chunk: &mut Chunk, asset_server: &mut AssetServer, sprite_sheet_idx: usize, chunk_x: u32, chunk_y: u32) {
    let mut row_idx = 0;
    let block_size = 0.2;
    let block_size_halfed = block_size / 2.;
    let rows = &chunk.rows;
    let mut vertices_main = vec![];
    let mut indices_main = vec![];
    for col in rows {
        let mut col_idx = 0;
        for block in col {
            let x = col_idx as f32 * block_size;
            let y = row_idx as f32 * block_size;
            println!("{},{}", x, y);
            let p1 = Vec2::new(-block_size_halfed + x, -block_size_halfed + y);
            let p2 = Vec2::new(block_size_halfed + x, block_size_halfed + y);
            let tex_coords = get_tex_coords(block);
            let (mut vertices, mut indices) = rect_with_tex_coords(p1, p2, tex_coords.0, tex_coords.1);
            vertices_main.append(&mut vertices);
            indices_main.append(&mut indices);
            col_idx += 1;
        }
        row_idx += 1;
    }
    asset_server.build_mesh(
        vertices_main,
        indices_main,
        vec![&mut Instance {position: Vec3::new(chunk_x as f32, chunk_y as f32, 0.),  ..Default::default()}],
        sprite_sheet_idx,
        false,
    );
}
fn get_tex_coords(block: &Block) -> (Vec2, Vec2) {
    let id = block.block_id;
    const NUM_SPRITES_IN_TEXTURE: u32 = 16; //must be perfect square
    const SPRITE_SIZE: f32 = 1.0 / (NUM_SPRITES_IN_TEXTURE as f32);

    let row = id / NUM_SPRITES_IN_TEXTURE;
    let col = id % NUM_SPRITES_IN_TEXTURE;

    let min_x = col as f32 * SPRITE_SIZE;
    let max_x = min_x + SPRITE_SIZE;
    let min_y = row as f32 * SPRITE_SIZE;
    let max_y = min_y + SPRITE_SIZE;
    (Vec2::new(min_x, min_y), Vec2::new(max_x,max_y))
}
#[derive(Clone, Copy)]
pub struct Block {
    pub block_id: u32, //this is used both for the blocktype and the index in the texture map
}
#[derive(Resource)]
pub struct World {
    pub world_width: usize,
    pub world_height: usize,
    pub block_ids_list: Vec<BlockType>,
    pub chunks: Vec<Vec<Chunk>>, //each row contains columns
    pub chest_locations: Vec<Chest>,
    pub sprite_map_idx: usize,
}
impl World {
    pub fn new(width: usize, height: usize, sprite_map_idx: usize) -> Self {
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
            sprite_map_idx
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
