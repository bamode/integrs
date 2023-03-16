#![allow(unused)]
use integrs::*;

fn main() {
    let lr = LeftRiemann;

    let result = lr.integrate(
        |x| (x + 1.).ln(),
        0.0,
        1.0,
    );

    println!("result: {result}");
}

