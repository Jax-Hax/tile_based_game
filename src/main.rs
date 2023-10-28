use glam::Vec3;
use tile_based_game::prelude::*;
use tile_game::terrain::gen;
mod tile_game{
    pub mod terrain;
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
    let (mut state, event_loop) = State::new(true, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //add models
    //custom mesh
    let mut world = gen(&mut state, 1000, 500);
    world.save_to_image("output.png");
    //render loop
    //run_event_loop(state, event_loop);
}