use crossterm::style::Color;
use pixel_blind::Canvas;
use std::{io::Write, thread, time::Duration};

fn main() {
    let width: u32 = 160;
    let height: u32 = 48;
    let mut canvas = Canvas::new(width, height);

    let amplitude = (height as f64 / 2.0) - 1.0;
    let center_y = height as f64 / 2.0;
    let frequency = 2.0 * std::f64::consts::PI / width as f64 * 2.0;

    let mut phase: f64 = 0.0;

    print!("\x1B[s\x1B[?25l");
    std::io::stdout().flush().unwrap();

    ctrlc::set_handler(|| {
        print!("\x1B[u\x1B[?25h");
        std::io::stdout().flush().unwrap();
        std::process::exit(0);
    })
    .unwrap();

    let p1 = (0, 0);
    let p2 = (width - 1, 0);
    let p3 = (width - 1, height - 1);
    let p4 = (0, height - 1);

    loop {
        canvas.clear();

        canvas.line(p1.0, p1.1, p2.0, p2.1);
        canvas.line(p2.0, p2.1, p3.0, p3.1);
        canvas.line(p3.0, p3.1, p4.0, p4.1);
        canvas.line(p4.0, p4.1, p1.0, p1.1);

        let mut rightmost_y = center_y as u32;
        for x in 0..width {
            let y_f = center_y + amplitude * (frequency * x as f64 + phase).sin();
            let y = y_f.round().clamp(0.0, (height - 1) as f64) as u32;
            canvas.set(x, y);

            if x == width - 1 {
                rightmost_y = y;
            }
        }

        canvas.line_colored(0, height - 1, width - 1, rightmost_y, Color::Blue);

        print!("\x1B[u{}", canvas.frame());
        std::io::stdout().flush().unwrap();

        phase -= 0.15;
        thread::sleep(Duration::from_millis(50));
    }
}
