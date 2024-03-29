use integrs::*;

use plotters::prelude::*;

fn main() {
    let integrator = Midpoint;

    let x_min = -10.0;
    let x_max = 10.0;
    let n_steps = 10000;

    let mut conv = Vec::with_capacity(n_steps);

    for i in 0..n_steps {
        let x_value = (x_max - x_min) * i as f64 / n_steps as f64 + x_min;
        let f = |x: f64| {
            if x != 0.0 {
                x.sin() / x * gaussian(x_value - x)
            } else {
                1.0 * gaussian(x_value - x)
            }
        };

        conv.push((
            x_value,
            integrator.integrate(f, -100.0, 100.0, n_steps) as f64,
        ));
    }

    plot_result(conv).unwrap();
}

#[inline]
fn gaussian(x: f64) -> f64 {
    (-1.0 * x * x).exp()
}

fn plot_result(data: Vec<(f64, f64)>) -> Result<(), Box<dyn std::error::Error>> {
    let root =
        BitMapBackend::new("gaussian-convolution-example.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Gaussian Sine Convolution", ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f64..10f64, -2f64..2f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(data, &RED))?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
