use pixel_blind::Canvas;
use std::{f32, io::Write, thread, time::Duration};

fn main() {
    let width: u32 = 80;
    let height: u32 = 80;
    let mut canvas = Canvas::new(width, height);
    let mut theta = 0.0;

    print!("\x1B[s\x1B[?25l");
    std::io::stdout().flush().unwrap();

    ctrlc::set_handler(|| {
        print!("\x1B[u\x1B[?25h");
        std::io::stdout().flush().unwrap();
        std::process::exit(0);
    })
    .unwrap();

    loop {
        canvas.clear();
        canvas.polygon(40, 40, 3, 20, theta);

        print!("\x1B[u{}", canvas.frame());
        std::io::stdout().flush().unwrap();
        theta += f32::consts::PI / 40.0;
        thread::sleep(Duration::from_millis(50));
    }
}
