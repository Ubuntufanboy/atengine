use std::io::{self, Write};

pub struct Pixel {
    fore: (u8, u8, u8),
    back: (u8, u8, u8),
    forea: u8,
    backa: u8,
}

impl Pixel {
    pub fn new(fore: (u8, u8, u8), back: (u8, u8, u8), forea: u8, backa: u8) -> Self {
        Pixel { fore, back, forea, backa }
    }

    pub fn get_ascii(&self, threshold: u8) -> Option<Vec<u8>> {
        if self.forea >= threshold && self.backa >= threshold {
            Some(format!(
                "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄\x1b[0m", 
                self.back.0, self.back.1, self.back.2, 
                self.fore.0, self.fore.1, self.fore.2
            ).into_bytes())
        } else if self.forea >= threshold {
            Some(format!(
                "\x1b[38;2;{};{};{}m▄\x1b[0m", 
                self.fore.0, self.fore.1, self.fore.2
            ).into_bytes())
        } else if self.backa >= threshold {
            Some(format!(
                "\x1b[48;2;{};{};{}m▄\x1b[0m", 
                self.back.0, self.back.1, self.back.2
            ).into_bytes())
        } else {
            None
        }
    }
}

pub struct Pos {
    x: u16,
    y: u16,
}

impl Pos {
    pub fn new(x: u16, y: u16) -> Self {
        Pos { x, y }
    }
}

pub fn blit(pixel: &Pixel, pos: &Pos) {
    if let Some(pix) = pixel.get_ascii(1) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let _ = handle.write_all(format!("\x1b[{};{}H", pos.y, pos.x).as_bytes());
        let _ = handle.write_all(&pix);
        let _ = handle.flush();
    }
}

fn main() {
    let pixel = Pixel::new((255, 0, 0), (0, 0, 255), 255, 255);
    let pos = Pos::new(10, 10);

    blit(&pixel, &pos);
}

