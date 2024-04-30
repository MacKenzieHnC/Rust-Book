use nalgebra::Point2;
use std::io;

fn main() {
    let span = get_span();
    let points = get_linkage(span);

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

fn get_linkage(span: f32) -> [Point2<f32>; 3] {
    let scale = span / 8.0;

    return [
        Point2::new(1.0 * scale, 2.0 * scale),
        Point2::new(3.0 * scale, 3.0 * scale),
        Point2::new(span / 2.0, span / 2.0),
    ];
}
