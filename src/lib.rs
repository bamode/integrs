pub mod mc;

pub trait Integrator {
    fn integrate<F>(&self, f: F, a: f64, b: f64, n_steps: usize) -> f64
    where
        F: Fn(f64) -> f64;
}

pub struct LeftRiemann;

impl Integrator for LeftRiemann {
    fn integrate<F>(&self, f: F, a: f64, b: f64, n_steps: usize) -> f64
    where
        F: Fn(f64) -> f64,
    {
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
    where
        F: Fn(f64) -> f64,
    {
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
    where
        F: Fn(f64) -> f64,
    {
        let step = (b - a) / n_steps as f64;
        let mut accum = 0.0;
        for i in 0..n_steps {
            accum += f(a + (i as f64 + 0.5) * step) * step;
        }

        accum
    }
}

pub trait Integrator2D {
    fn integrate<F>(
        &self,
        f: F,
        x_range: (f64, f64),
        y_range: (f64, f64),
        x_steps: usize,
        y_steps: usize,
    ) -> f64
    where
        F: Fn(f64, f64) -> f64;
}

pub struct LeftRiemann2D;

impl Integrator2D for LeftRiemann2D {
    fn integrate<F>(
        &self,
        f: F,
        x_range: (f64, f64),
        y_range: (f64, f64),
        x_steps: usize,
        y_steps: usize,
    ) -> f64
    where
        F: Fn(f64, f64) -> f64,
    {
        let x_step = (x_range.1 - x_range.0) / x_steps as f64;
        let y_step = (y_range.1 - y_range.0) / y_steps as f64;
        let mut accum = 0.0;

        for j in 0..y_steps {
            for i in 0..x_steps {
                accum += f(x_range.0 + i as f64 * x_step, y_range.0 + j as f64 * y_step)
                    * x_step
                    * y_step;
            }
        }

        accum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2d_left_riemann_accuracy() {
        let lr2d = LeftRiemann2D;

        let res = lr2d.integrate(|x, y| x + y, (0.0, 1.0), (0.0, 1.0), 10000, 10000);

        assert!((1.0 - res).abs() < 1e-3);
    }
}
