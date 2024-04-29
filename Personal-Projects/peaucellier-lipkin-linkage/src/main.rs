use nalgebra::Point2;
use std::io;

fn main() {
    let span = get_span();
}

fn get_span() -> f32 {
    println!("How far does the linkage need to travel?");

    let mut span = String::new();

    io::stdin()
        .read_line(&mut span)
        .expect("Failed to read line");

    let span: f32 = match span.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("span is not a number: {:?}", e),
    };
    return span;
}
