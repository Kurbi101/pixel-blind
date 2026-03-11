use pixel_blind::Canvas;
use crossterm::style::Color;

fn main() {
    let mut canvas = Canvas::new(100, 100);
    canvas.line_colored(2, 2, 80, 80, Color::Red);
    canvas.line_colored(2, 80, 80, 80, Color::Green);
    canvas.line_colored(2, 2, 2, 80, Color::Blue);

    canvas.line_colored(
        2 + 5,
        2 + 15,
        80 + 5,
        80 + 15,
        Color::Rgb { r: 255, g: 0, b: 0 },
    );
    canvas.line_colored(
        2 + 5,
        80 + 15,
        80 + 5,
        80 + 15,
        Color::Rgb { r: 0, g: 255, b: 0 },
    );
    canvas.line_colored(
        2 + 5,
        2 + 15,
        2 + 5,
        80 + 15,
        Color::Rgb { r: 0, g: 0, b: 255 },
    );
    println!("{}", canvas.frame());
}