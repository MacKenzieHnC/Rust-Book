use nalgebra::Point2;
use std::io;

fn main() {
    println!("How tall is the linkage?");

    let mut h = String::new();

    io::stdin().read_line(&mut h).expect("Failed to read line");

    let h: f32 = match h.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("h is not a number: {:?}", e),
    };

    println!("The linkage is {h} tall!")
}
