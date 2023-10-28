use tile_based_game::material::Material;

pub fn gen() -> World {
    let mut world = World {
        world_width: 100,
        world_height: 50,
        block_ids_list: vec![],
        block_world: vec![vec![Block{block_id: 0};100]; 50],
        chest_locations: vec![],
    };
    dirt_pass(&mut world);
    world
}
fn dirt_pass(world: &mut World) {}
#[derive(Clone,Copy)]
pub struct Block {
    pub block_id: u32,
}
pub struct World {
    pub world_width: u32,
    pub world_height: u32,
    pub block_ids_list: Vec<BlockType>,
    pub block_world: Vec<Vec<Block>>, //each row contains columns
    pub chest_locations: Vec<Chest>,
}
pub struct Chest {}
pub struct BlockType {
    pub mining_power: u16,
    pub block_sprite: Material,
}
