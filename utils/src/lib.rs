use rand::Rng;

pub const PI: f64 = core::f64::consts::PI;
pub const INFINITY: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.
}

/// Returns a random real in [0, 1).
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
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

#[cfg(test)]
mod test {
    use crate::random_double;

    #[test]
    fn test_random_double() {
        let v1 = random_double();
        let v2 = random_double();
        assert_ne!(v1, v2);
    }
}
