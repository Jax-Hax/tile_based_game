use glam::{Vec2, Vec3};
use tile_based_game::{state, prelude::{Instance, run_event_loop}, resources::MousePos};
use {
    tile_based_game::camera::{Camera},
    tile_based_game::collision::Box2D,
    tile_based_game::prelude::*,
    tile_based_game::primitives::rect,
};
use bevy_ecs::prelude::*;
fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
    );
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = state::State::new(false, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //custom mesh
    let p1 = Vec2::new(-0.5, -0.5);
    let p2 = Vec2::new(0.5, 0.5);
    let (vertices, indices) = rect(p1,p2);
    let collider = Box2D::new(p1,p2);
    let mut instance = Instance {position: Vec3::new(2.0,1.0,0.0), ..Default::default()};
    let mut instance2 = Instance {position: Vec3::new(1.0,1.0,0.0), ..Default::default()};
    let mut instances = vec![];
    instances.push(&mut instance);
    instances.push(&mut instance2);
    state.build_mesh(
        vertices,
        indices,
        instances,
        state.compile_material("rounded_rect.png").await,
        false,
    );
    state.world.spawn((instance, collider));
    //render loop
    run_event_loop(state, event_loop);
}