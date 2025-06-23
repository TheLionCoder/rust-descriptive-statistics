pub mod feature_engineering {
    pub fn experience_level_score(level: &str) -> u8 {
        match level {
            "EX" => 0,
            "SE" => 1,
            "EN" => 2,
            _ => 3,
        }
    }
}
