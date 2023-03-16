#![allow(unused)]
use integrs::{LeftRiemann, RightRiemann, Midpoint, Integrator};

fn main() {
    let lr = LeftRiemann;
    let rr = RightRiemann;
    let mp = Midpoint;


    let result = lr.integrate(
        |x| x.sin(),
        0.0,
        std::f64::consts::PI / 2.0,
        10000,
    );

    let result2 = rr.integrate(
        |x| x.sin(),
        0.0,
        std::f64::consts::PI / 2.0,
        10000,
    );

    let result3 = mp.integrate(
        |x| x.sin(),
        0.0,
        std::f64::consts::PI / 2.0,
        10000,
    );

    println!("result: {result}");
    println!("result2: {result2}");
    println!("result3: {result3}");
}

