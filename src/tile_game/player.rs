use bevy_ecs::system::Resource;

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