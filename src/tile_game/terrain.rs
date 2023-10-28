use glam::{Vec3, Vec2};
use tile_based_game::{material::Material, prelude::Instance, state::State, primitives::rect};

pub async fn gen(state: &mut State) -> World {
    let mut world = World {
        world_width: 100,
        world_height: 50,
        block_ids_list: vec![],
        block_world: vec![vec![Block{block_id: 0};100]; 50],
        chest_locations: vec![],
    };
    dirt_pass(&mut world);
    render_pass(&mut world, state).await;
    world
}
fn dirt_pass(world: &mut World) {}
async fn render_pass(world: &mut World, state: &mut State) {
    let mut row_idx = 0;
    let mut instances = vec![];
    let rows = &world.block_world;
    for col in rows{
        let mut col_idx = 0;
        for block in col {
            let instance = Instance {
                position: Vec3::new(row_idx as f32, col_idx as f32, 0.), //change to swithc pos
                ..Default::default()
            };
            instances.push(instance);
            println!("{row_idx}, {col_idx}");
            col_idx += 1;
        }
        row_idx += 1;
    }
    let p1 = Vec2::new(-0.5, -0.5);
    let p2 = Vec2::new(0.5, 0.5);
    let (vertices, indices) = rect(p1,p2);
    state.build_mesh(
        vertices,
        indices,
        instances.iter_mut().map(|instance| instance).collect(),
        state.compile_material("cube-diffuse.jpg").await,
        false,
    );
}
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
