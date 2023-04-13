#[allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn square(point: &Point, length: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y + length,
        },
        bottom_right: Point {
            x: point.x + length,
            y: point.y,
        },
    }
}

fn main() {
    println!("square: {:?}", square(&Point { x: 0.0, y: 0.0 }, 1.0));
}
