use glam::Vec3;
use tile_based_game::prelude::*;
use tile_game::{terrain::{gen, chunk_render_checker}, ui::gen_new_world_btn};
mod tile_game{
    pub mod terrain;
    pub mod terrain_passes;
    pub mod ui;
    pub mod player;
}
fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0)
    );
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop, world, schedule) = State::new(false, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //add models
    //custom mesh
    let mut terrain_world = gen(1000, 500, 1);
    terrain_world.save_to_image("output.png");
    world.insert_resource(terrain_world);
    schedule.add_systems(chunk_render_checker);
    //render loop
    
    gen_new_world_btn(&mut state).await;
    
    
    run_event_loop(state, event_loop, world, schedule);
}