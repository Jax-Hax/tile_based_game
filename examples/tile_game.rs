use bevy_ecs::system::{Query, Res, ResMut};
use glam::{Quat, Vec3, Vec2};
use tile_based_game::{prelude::*, primitives::rect, collision::Box2D};

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
    let p1 = Vec2::new(0., 0.);
    let p2 = Vec2::new(0.5, 0.5);
    let (vertices, indices) = rect(p1,p2);
    let collider = Box2D::new(p1,p2);
    let mut instance = Instance { ..Default::default()};
    let mut instances = vec![];
    instances.push(&mut instance);
    state.build_mesh(
        vertices,
        indices,
        instances,
        state.compile_material("cube-diffuse.jpg").await,
        false,
    );
    //render loop
    run_event_loop(state, event_loop);
}
fn movement(
    mut query: Query<(&mut Instance,)>,
    mut instance_update: ResMut<UpdateInstance>,
    delta_time: Res<DeltaTime>,
) {
    let mut instances = vec![];
    let mut temp_instance = Instance {
        ..Default::default()
    };
    for (mut instance,) in &mut query {
        instance.position[0] += 10. * delta_time_to_seconds(delta_time.dt);
        let instance_raw = instance.to_raw();
        if instance_raw.is_some() {
            instances.push(instance_raw.unwrap());
        }
        temp_instance = *instance;
    }
    temp_instance.update(instances, &mut instance_update);
}
fn movement_with_key(
    mut query: Query<(&mut Instance,)>,
    mut instance_update: ResMut<UpdateInstance>,
    delta_time: Res<DeltaTime>,
    window_events: Res<WindowEvents>,
) {
    if window_events.is_key_pressed(VirtualKeyCode::D, None) {
        let mut instances = vec![];
        let mut temp_instance = Instance {
            ..Default::default()
        };
        for (mut instance,) in &mut query {
            instance.position[1] += 50. * delta_time_to_seconds(delta_time.dt);
            let instance_raw = instance.to_raw();
            if instance_raw.is_some() {
                instances.push(instance_raw.unwrap());
            }
            temp_instance = *instance;
        }
        temp_instance.update(instances, &mut instance_update);
    }
}
