pub mod noise;

pub trait TerrainGenerator {
    fn get_block_type(&self, height: f64) -> u8;
}

pub struct DebugTerrainGenerator;

impl TerrainGenerator for DebugTerrainGenerator {
    fn get_block_type(&self, height: f64) -> u8 {
        for (id, range) in [(1, 7..40), (2, 5..7), (3, 4..5), (4, 2..4), (3, 0..2)] {
            if range.contains(&(height.ceil() as i32)) {
                return id;
            }
        }

        return 0;
    }
}
