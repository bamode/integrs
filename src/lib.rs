pub trait Integrator {
    fn integrate<F>(&self, f: F, a: f64, b: f64, n_steps: usize) -> f64
    where F: Fn(f64) -> f64;
}

pub struct LeftRiemann;

impl Integrator for LeftRiemann {
    fn integrate<F>(&self, f: F, a: f64, b: f64, n_steps: usize) -> f64
    where F: Fn(f64) -> f64 {
        let step = (b - a) / n_steps as f64;
        let mut accum = 0.0;
        for i in 0..n_steps {
            accum += f(a + i as f64 * step) * step;    
        }

        accum
    }
}

pub struct RightRiemann;

impl Integrator for RightRiemann {
    fn integrate<F>(&self, f: F, a: f64, b: f64, n_steps: usize) -> f64
    where F: Fn(f64) -> f64 {
        let step = (b - a) / n_steps as f64;
        let mut accum = 0.0;
        for i in 1..=n_steps {
            accum += f(a + i as f64 * step) * step;    
        }

        accum
    }
}

pub struct Midpoint;

impl Integrator for Midpoint {
    fn integrate<F>(&self, f: F, a: f64, b: f64, n_steps: usize) -> f64
    where F: Fn(f64) -> f64 {
        let step = (b - a) / n_steps as f64;
        let mut accum = 0.0;
        for i in 0..n_steps {
            accum += f(a + (i as f64 + 0.5) * step) * step;    
        }

        accum
    }
}
