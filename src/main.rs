#![allow(unused)]
use integrs::{Integrator, LeftRiemann, Midpoint, RightRiemann};

fn main() {
    let truth = 1.0;

    let scheme = LeftRiemann;
    let result = scheme.integrate(|x| x.sin(), 0.0, std::f64::consts::PI / 2.0, 10000);

    let scheme = RightRiemann;
    let result2 = scheme.integrate(|x| x.sin(), 0.0, std::f64::consts::PI / 2.0, 10000);

    let scheme = Midpoint;
    let result3 = scheme.integrate(|x| x.sin(), 0.0, std::f64::consts::PI / 2.0, 10000);

    println!(
        "Right Riemann: {result}, residual: {residual}",
        result = result,
        residual = (result - truth).abs()
    );
    println!(
        "Left Riemann: {result2}, residual: {residual}",
        result2 = result2,
        residual = (result2 - truth).abs()
    );
    println!(
        "Midpoint: {result3}, residual: {residual}",
        result3 = result3,
        residual = (result3 - truth).abs()
    );
}
