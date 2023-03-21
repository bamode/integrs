use rand::prelude::*;

pub struct MonteCarlo2D {
    rng: ThreadRng,
}

impl MonteCarlo2D {
    pub fn new_with_rng() -> Self {
        Self { rng: thread_rng() }
    }

    pub fn integrate<F>(
        &mut self,
        f: F,
        x_range: (f64, f64),
        y_range: (f64, f64),
        iterations: usize,
    ) -> f64
    where
        F: Fn(f64, f64) -> f64,
    {
        let mut accum = 0.0;

        // do stuff
        let v = (x_range.1 - x_range.0).abs() * (y_range.1 - y_range.0).abs();
        let n = iterations as f64;

        for _ in 0..iterations {
            let x = self.rng.gen_range(x_range.0..=x_range.1);
            let y = self.rng.gen_range(y_range.0..=x_range.1);
            accum += f(x, y);
        }

        v * accum / n
    }

    pub fn integrate_and_return_points<F>(
        &mut self,
        f: F,
        x_range: (f64, f64),
        y_range: (f64, f64),
        iterations: usize,
    ) -> (Vec<(f64, f64)>, f64) 
    where F: Fn(f64, f64) -> f64
    {
        let mut accum = 0.0;
        let mut points = Vec::with_capacity(iterations);

        let v = (x_range.1 - x_range.0).abs() * (y_range.1 - y_range.0).abs();
        let n = iterations as f64;

        for _ in 0..iterations {
            let x = self.rng.gen_range(x_range.0..=x_range.1);
            let y = self.rng.gen_range(y_range.0..=x_range.1);
            
            points.push((x, y));
            accum += f(x, y);
        }

        (points, v * accum / n)

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monte_carlo_2d() {
        let h = |x, y| {
            let squared_distance = x * x + y * y;
            if squared_distance <= 1.0 {
                1.0
            } else {
                0.0
            }
        };

        let mut mc = MonteCarlo2D::new_with_rng();

        let res = mc.integrate(h, (-1.0, 1.0), (-1.0, 1.0), 100000);

        println!("res: {res}");
        println!("target: {:.05}", std::f64::consts::PI);
    }
}
