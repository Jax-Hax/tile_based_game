pub fn gen() -> World {
    let world = World {
        world_width: 100,
        world_height: 50,
        block_ids_list: todo!(),
        block_world: todo!(),
        chest_locations: todo!(),
    };
    dirt_pass(&mut world);
}
fn dirt_pass(world: &mut World) {

}
pub struct Block {
    pub block_id: u32
}
pub struct World{
    pub world_width: u32,
    pub world_height: u32,
    pub block_ids_list: Vec<BlockType>,
    pub block_world: Vec<Vec<Block>>, //each row contains columns
    pub chest_locations: Vec<Chest>
}
pub struct Chest{

}
pub struct BlockType{
    pub mining_power: u16,
    pub block_sprite: Material,
}