use plotly::common::{Mode, Title};
use plotly::layout::Layout;
use plotly::plot::ImageFormat;
use plotly::{Plot, Scatter};

use integrs::mc::MonteCarlo2D;

fn main() {
    let h = |x, y| {
        let squared_distance = x * x + y * y;
        if squared_distance <= 1.0 {
            1.0
        } else {
            0.0
        }
    };

    let mut mc = MonteCarlo2D::new_with_rng();

    let (points, res) = mc.integrate_and_return_points(h, (-1.0, 1.0), (-1.0, 1.0), 100000);

    println!("finished calculating");

    let points_in = points
        .iter()
        .map(|x| *x)
        .filter(|(x, y)| x * x + y * y <= 1.0)
        .collect::<Vec<(f64, f64)>>();
    let points_out = points
        .iter()
        .map(|x| *x)
        .filter(|(x, y)| x * x + y * y > 1.0)
        .collect::<Vec<(f64, f64)>>();

    let trace_in = Scatter::new(
        points_in.iter().map(|(x, _)| *x).collect(),
        points_in.iter().map(|(_, y)| *y).collect(),
    )
    .mode(Mode::Markers)
    .name("Inner Points");

    let trace_out = 
    Scatter::new(
        points_out.iter().map(|(x, _)| *x).collect(),
        points_out.iter().map(|(_, y)| *y).collect(),
    )
    .mode(Mode::Markers)
    .name("Outer Points");

    let layout = Layout::new().title(Title::new(&format!("Monte Carlo Calculation of Pi: {res}")));
    let mut plot = Plot::new();
    plot.add_trace(trace_in);
    plot.add_trace(trace_out);
    plot.set_layout(layout);
    plot.write_image("mc-pi.png", ImageFormat::PNG, 950, 900, 1.0);
}
