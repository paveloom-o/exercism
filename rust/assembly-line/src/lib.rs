// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// #![allow(unused)]

#[must_use]
pub fn production_rate_per_hour(speed: u8) -> f64 {
    221.0
        * f64::from(speed)
        * match speed {
            1..=4 => 1.0,
            5..=8 => 0.9,
            9..=10 => 0.77,
            _ => 0.0,
        }
}

#[must_use]
pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
