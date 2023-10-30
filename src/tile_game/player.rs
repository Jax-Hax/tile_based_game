use bevy_ecs::system::{Resource, Query, ResMut, Res};
use tile_based_game::resources::WindowEvents;
use winit::event::VirtualKeyCode;

use super::terrain::World;

#[derive(Resource)]
pub struct Player {
    pub position: (f32,f32)
}
impl Player {
    pub fn block_position(&self) -> (u32,u32) {
        let (x,y) = self.position;
        (x.round() as u32,y.round() as u32)
    }
}
fn player_movement(window_events: Res<WindowEvents>, mut world: ResMut<World>, mut asset_server: ResMut<Player>) {
    if window_events.is_key_pressed(VirtualKeyCode::D, None) {
        
    }
    /*if collider.check_collision(instance, &window_events) && window_events.left_clicked() {
        for chunk_row in &world.chunks {
            for chunk in chunk_row {
                if chunk.rendered {println!("{}", chunk.prefab_idx);asset_server.remove_prefab(chunk.prefab_idx);}
            }
        }
        let mut world = gen(1000, 500, rand::thread_rng().gen_range(0..100000), world.sprite_map_idx);
        world.save_to_image("output.png");
    }*/
}