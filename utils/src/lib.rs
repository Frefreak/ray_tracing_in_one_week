use rand::Rng;

pub const PI: f64 = core::f64::consts::PI;
pub const INFINITY: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.
}

/// Returns a random real in [0, 1).
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    let r = rng.gen::<f64>();
    r / (INFINITY + 1.)
}

/// Returns a random real in [min, max).
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
