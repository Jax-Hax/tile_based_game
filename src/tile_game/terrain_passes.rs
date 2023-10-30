use noise::{core::simplex::simplex_2d, permutationtable::PermutationTable};

use super::terrain::World;
fn noise_2d_int(hasher: &PermutationTable, x: usize, y: usize, magnitude: f64, variance: f64, offset: usize) -> usize {
    let noise = simplex_2d([(x as f64 + 0.5)/magnitude, (y as f64 + 0.5)/magnitude],hasher).0;
    ((noise+1.)/2. * variance) as usize + offset
}
fn noise_2d_zero_to_one(hasher: &PermutationTable, x: usize, y: usize, magnitude: f64) -> f64 {
    simplex_2d([(x as f64 + 0.5)/magnitude, (y as f64 + 0.5)/magnitude],hasher).0
}
pub fn basic_caves_pass(world: &mut World, hasher: &PermutationTable) {
    for col in 0..world.world_width {
        let dirt_val = noise_2d_int(hasher, col, 1, 10., 25., 50);
        let stone_val = noise_2d_int(hasher, col, 1, 10., 25., 100);
        for row in 0..world.world_height {
            let block = world.get_block(row, col).unwrap();
            if row < dirt_val{
                block.block_id = 0; //sky
            } else if row < stone_val {
                block.block_id = 1; //dirt
            }
            else {
                block.block_id = 2; //stone
                //check if cave
                let cave_val = noise_2d_zero_to_one(hasher, col,row, 25.);
                if cave_val > 0.1 { //TODO: ADD FRACTAL NOISE, ADD MULTIPLE SMALLER LAYERS OF NOISE FOR REFINED DETAILS
                    block.block_id = 0;
                }
            }
            
        }
    }
}
