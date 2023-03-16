pub trait Integrator {
    fn integrate<F>(&self, f: F, a: f64, b: f64) -> f64
    where F: Fn(f64) -> f64;
}

pub struct LeftRiemann;

impl Integrator for LeftRiemann {
    fn integrate<F>(&self, f: F, a: f64, b: f64) -> f64
    where F: Fn(f64) -> f64 {
        let step = (b - a) / 100.0;
        let mut accum = 0.0;
        for i in 0..100 {
            accum += f(a + i as f64 * step) * step;    
        }

        accum
    }
}