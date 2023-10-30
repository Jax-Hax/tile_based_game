use bevy_ecs::system::{Query, Res};
use glam::Vec2;
use rand::Rng;
use tile_based_game::{state::State, prelude::*, primitives::rect, collision::Box2D, assets::AssetServer};

use super::terrain::{gen, World};

pub async fn gen_new_world_btn(state: &mut State) {
    let p1 = Vec2::new(-0.7, -0.4);
    let p2 = Vec2::new(-0.9, -0.5);
    let (vertices, indices) = rect(p1,p2);
    let collider = Box2D::new(p1,p2);
    let mut instance = Instance {is_world_space: false, ..Default::default()};
    let mut instances = vec![];
    instances.push(&mut instance);
    let mut asset_server = state.world.get_resource_mut::<AssetServer>().unwrap();
    let rect_idx = asset_server.compile_material("rounded_rect.png").await;
    asset_server.build_mesh(
        vertices,
        indices,
        instances,
        rect_idx,
        false,
    );
    state.world.spawn((instance, collider));
    state.schedule.add_systems(check_collisions);
}
fn check_collisions(query: Query<(&Instance, &Box2D)>, window_events: Res<WindowEvents>, world: Res<World>) {
    for (instance, collider) in &query {
        if collider.check_collision(instance, &window_events) && window_events.left_clicked() {
            let mut world = gen(1000, 500, rand::thread_rng().gen_range(0..100000), world.sprite_map_idx);
            world.save_to_image("output.png");
        }
    }
}