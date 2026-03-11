use std::collections::HashMap;

use crossterm::style::{Color, Stylize};

static PIXELS: [[u8; 2]; 4] = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]];
fn get_pixel(x: u32, y: u32) -> u8 {
    PIXELS[y as usize % 4][x as usize % 2]
}
const BRAILLE_START: u32 = 0x2800;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pixel {
    pixels: u8,
    char: char,
    color: Color,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            pixels: 0,
            char: ' ',
            color: Color::White,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    x: u32,
    y: u32,
}

impl Pos {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x: x, y: y }
    }

    pub fn normalize(x: u32, y: u32) -> Self {
        Self::new(x / 2, y / 4)
    }
}

fn lerp(t: f64, x1: u32, y1: u32, x2: u32, y2: u32) -> (f64, f64) {
    let x_diff = x2 as f64 - x1 as f64;
    let y_diff = y2 as f64 - y1 as f64;
    (x1 as f64 + t * x_diff, y1 as f64 + t * y_diff)
}

pub struct Canvas {
    data: HashMap<Pos, Pixel>,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            data: HashMap::new(),
            width: width / 2,
            height: height / 4,
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_pos_valid(&self, x: u32, y: u32) -> bool {
        let result = x < self.width * 2 && y < self.height * 4;
        debug_assert!(
            result,
            "Pos ({}, {}) is out of bounds for canvas of size {} x {}",
            x,
            y,
            self.height * 4,
            self.width * 2
        );
        result
    }

    pub fn set_colored(&mut self, x: u32, y: u32, color: Color) {
        if !self.is_pos_valid(x, y) {
            return;
        }
        let pos = Pos::normalize(x, y);
        let pixel = self.data.entry(pos).or_insert(Pixel::default());
        pixel.pixels |= PIXELS[y as usize % 4][x as usize % 2];
        pixel.color = color;
        pixel.char = ' ';
    }

    pub fn set(&mut self, x: u32, y: u32) {
        self.set_colored(x, y, Color::White);
    }

    pub fn set_char_colored(&mut self, x: u32, y: u32, c: char, color: Color) {
        if !self.is_pos_valid(x, y) {
            return;
        }
        let pos = Pos::normalize(x, y);
        let pixel = self.data.entry(pos).or_insert(Pixel::default());
        pixel.pixels |= PIXELS[y as usize % 4][x as usize % 2];
        pixel.pixels = 0;
        pixel.char = c;
        pixel.color = color;
    }

    pub fn set_char(&mut self, x: u32, y: u32, c: char) {
        self.set_char_colored(x, y, c, Color::White);
    }

    pub fn text_colored(&mut self, x: u32, y: u32, text: &str, color: Color) {
        if !self.is_pos_valid(x, y) {
            return;
        }
        text.char_indices()
            .map(|(offset, c)| (offset as u32 * 2, c))
            .for_each(|(offset, c)| self.set_char_colored(x + offset, y, c, color));
    }

    pub fn text(&mut self, x: u32, y: u32, text: &str) {
        self.text_colored(x, y, text, Color::White);
    }

    pub fn unset(&mut self, x: u32, y: u32) {
        if !self.is_pos_valid(x, y) {
            return;
        }
        let pos = Pos::normalize(x, y);
        let pixel = self.data.entry(pos).or_insert(Pixel::default());
        pixel.pixels &= get_pixel(x, y);
    }

    pub fn toggle(&mut self, x: u32, y: u32) {
        if !self.is_pos_valid(x, y) {
            return;
        }
        let pos = Pos::normalize(x, y);
        let pixel = self.data.entry(pos).or_insert(Pixel::default());
        pixel.pixels ^= get_pixel(x, y);
    }

    pub fn get(&self, x: u32, y: u32) -> bool {
        if !self.is_pos_valid(x, y) {
            return false;
        }
        let pos = Pos::normalize(x, y);
        self.data.get(&pos).map_or(false, |a| {
            let dot = get_pixel(x, y);
            a.pixels & dot != 0
        })
    }

    pub fn line_colored(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: Color) {
        const ACCURACY: usize = 100;

        for t in 0..=ACCURACY {
            let (x, y) = lerp(t as f64 / ACCURACY as f64, x1, y1, x2, y2);
            self.set_colored(x as u32, y as u32, color);
        }
    }

    pub fn line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        self.line_colored(x1, y1, x2, y2, Color::White);
    }

    pub fn row(&self) -> Vec<String> {
        let max_row = self
            .width
            .max(self.data.keys().map(|pos| pos.x).max().unwrap_or(0));
        let max_col = self
            .height
            .max(self.data.keys().map(|pos| pos.y).max().unwrap_or(0));

        let mut result = Vec::with_capacity(max_col as usize + 1);

        for y in 0..max_col {
            let mut row = String::with_capacity(max_row as usize + 1);
            for x in 0..max_row {
                let cell = self
                    .data
                    .get(&(Pos::new(x, y)))
                    .cloned()
                    .unwrap_or(Pixel::default());
                if cell.pixels == 0 {
                    row.push(cell.char);
                } else {
                    row.push_str(&format!(
                        "{}",
                        char::from_u32(BRAILLE_START + cell.pixels as u32)
                            .unwrap()
                            .with(cell.color)
                    ));
                }
            }
            result.push(row);
        }
        result
    }

    pub fn frame(&self) -> String {
        self.row().join("\n")
    }
}
