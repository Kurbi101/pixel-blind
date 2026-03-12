use pixel_blind::{Canvas, Turtle};

fn main() {
    let canvas = Canvas::new(400, 300);
    let mut turtle = Turtle::new(150.0, 150.0, canvas);
    for _ in 0..36 {
        turtle.right(10.0);
        for _ in 0..36 {
            turtle.right(10.0);
            turtle.forward(10.0);
        }
    }
    
    turtle.print();
}
