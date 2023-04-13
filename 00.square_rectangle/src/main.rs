

#[derive(Debug)] 
struct Point{

  x:f32,
  y:f32,
}


#[derive(Debug)] 
struct Rectangle {
    top_left: Point,
    bottom_right:Point,
}

fn square_rectangle(point: &Point, l:f32)-> Rectangle{

  Rectangle { 
    top_left: Point {

    x: point.x,
    y: point.y+l,

  },
  bottom_right: Point 
  { x: (point.x+l), 
    y: (point.y) },
}
}

fn main (){    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    let shape= Point{x:10.0, y:10.0};
    println!("The rectangle is: {:?}", square_rectangle(&shape, 15.0));

}