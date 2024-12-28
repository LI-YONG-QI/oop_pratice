pub struct LevelSheet {}

impl LevelSheet {
    pub fn new() -> Self {
        LevelSheet {}
    }

    pub fn query_level(&self, exp: u32) -> u32 {
        exp / 1000 + 1
    }
}
