#![allow(unused)]
use integrs::*;

fn main() {
    let lr = LeftRiemann;

    let result = lr.integrate(
        |x| x.sin(),
        0.0,
        std::f64::consts::PI / 2.0,
        10000,
    );

    println!("result: {result}");
}

