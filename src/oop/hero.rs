use super::level_sheet::LevelSheet;

pub struct Hero {
    level: u32,
    total_exp: u32,
    hp: u32,
}

impl Hero {
    pub fn new() -> Self {
        Hero {
            level: 1,
            total_exp: 0,
            hp: 100,
        }
    }

    pub fn gain_exp(&mut self, exp: u32, level_sheet: &LevelSheet) {
        self.total_exp += exp;

        let new_level = level_sheet.query_level(self.total_exp);
        self.set_level(new_level);
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    fn set_level(&mut self, new_level: u32) {
        self.level = new_level;
    }
}
