mod utils;
use utils::funcs::{add_five,subtract_ten};

fn main() {
    let mut x: u32 = 50;
    println!("x: {}", x);

    let y = add_five(x);
    println!("y: {}", y);

    x = 70;
    println!("x: {}", x);

    let z = subtract_ten(x);
    println!("z: {}", z);
}
