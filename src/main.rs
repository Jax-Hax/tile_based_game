use glam::Vec3;
use tile_based_game::{prelude::*, assets::AssetServer};
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
    let (mut state, event_loop) = State::new(false, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //add models
    //custom mesh
    let sprite_map_idx = state.world.get_resource_mut::<AssetServer>().unwrap().queue_material("cube-diffuse.jpg");
    let mut world = gen(1000, 500, 1, sprite_map_idx);
    world.save_to_image("output.png");
    state.world.insert_resource(world);
    state.schedule.add_systems(chunk_render_checker);
    //render loop
    
    gen_new_world_btn(&mut state);
    
    
    run_event_loop(state, event_loop);
}