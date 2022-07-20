use std::collections::HashMap;
use num::complex::Complex;

pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

pub type LivingCell = HashMap<Coordinates, bool>;
pub type Configuration = Vec<LivingCell>;

pub struct Universe {
    pub amplitude: Complex<f64>,
    pub configuration: Configuration,
}
