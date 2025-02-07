use numru::{arr, Axis};
use std::f64::consts::{E, PI, TAU};

fn main() {
    let a = arr![42, -17, 256, 3, 99, -8];
    println!("1D Shape={:?}", a.shape());
    a.visualize();
    let a_max = a.max(None);
    println!("Max={:?}", a_max);

    let b = arr![[TAU, -PI, 1.61], [E, 0.98, -7.42], [4.67, -0.45, 8.88]];
    println!("2D Shape={:?}", b.shape());
    b.visualize();
    let b_max = b.max(None);
    println!("Max={:?}", b_max);

    let c = arr![
        [[101, 202, 303], [404, 505, 606]],
        [[-707, -808, -909], [111, 222, 333]]
    ];
    println!("3D Shape={:?}", c.shape());
    c.visualize();
    let c_max = c.max(None);
    println!("Max={:?}", c_max);
}
