use nalgebra::Point2;
use std::io;

fn main() {
    let span = get_span();

    // try heuristic, works well for numbers divisible by 12 or sufficiently large numbers
    let points: [Point2<f32>; 3] = [
        Point2::new(span / 16.0, 5.0 * span / 24.0),
        Point2::new(13.0 * span / 48.0, 13.0 * span / 48.0),
        Point2::new(span / 2.0, span / 2.0),
    ];

    println!("key points:");
    for p in points {
        println!("{p}");
    }
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
