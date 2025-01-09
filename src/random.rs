use rand::prelude::*;

pub fn random_f64() -> f64 { // W: function `random_f64` is never used
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen::<f64>();
    y
}

pub fn random_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen_range(min..max);
    y
}
