use noise::{Perlin, NoiseFn};

use super::terrain::World;

pub fn dirt_pass(world: &mut World, perlin: Perlin) {
    let dirt_variance = 50.;
    for col in 0..world.world_width {
        let val = ((perlin.get([(col as f64 + 0.5)/10.,1.5])+1.)/2. * dirt_variance) as usize;
        println!("{}",val);
        for row in 0..world.world_height {
            let block = world.get_block(row, col).unwrap();
            if row < val{
                block.block_id = 0;
            } else if row < 100 {
                block.block_id = 1
            }
            else {
                block.block_id = 2;
            }
        }
    }
}