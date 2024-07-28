use image::{GenericImageView, DynamicImage};
use crate::blit::{Pixel, Pos, blit};
use termion::terminal_size;

pub struct Frame {
    pub odds: Vec<Vec<(u8, u8, u8, u8)>>,
    pub evens: Vec<Vec<(u8, u8, u8, u8)>>,
}

pub fn get_terminal_size() -> (u16, u16) {
    terminal_size().unwrap_or((80, 24)) // Default to 80x24 if unable to get terminal size
}

fn convert_to_u32_tuple(tuple: (u16, u16)) -> (u32, u32) {
    (tuple.0 as u32, tuple.1 as u32)
}

impl Frame {
    pub fn new(odds: Vec<Vec<(u8, u8, u8, u8)>>, evens: Vec<Vec<(u8, u8, u8, u8)>>) -> Self {
        Frame { odds, evens }
    }

    pub fn print_whole_frame(&self) {
        for (row_number, row) in self.evens.iter().enumerate() {
            for (pixel_number_in_row, &fore) in row.iter().enumerate() {
                let fore_alpha = fore.3;
                let back = self.odds[row_number][pixel_number_in_row];
                let pix = Pixel::new((fore.0, fore.1, fore.2), (back.0, back.1, back.2), fore_alpha, back.3);
                let pos = Pos::new(pixel_number_in_row as u16, row_number as u16);
                blit(&pix, &pos);
            }
        }
    }
}

pub fn img_to_frame(img_path: &str) -> Frame {
    let img = image::open(img_path).expect("Failed to open image");
    let (term_width, term_height) = convert_to_u32_tuple(get_terminal_size()); // Replace with your terminal size
    let img = img.resize_exact(term_width, term_height * 2, image::imageops::FilterType::Nearest);

    let mut odds = vec![vec![(0, 0, 0, 0); term_width as usize]; term_height as usize];
    let mut evens = vec![vec![(0, 0, 0, 0); term_width as usize]; term_height as usize];

    for y in (0..img.height()).step_by(2) {
        for x in 0..img.width() {
            let even_pixel = img.get_pixel(x, y);
            evens[y as usize / 2][x as usize] = (even_pixel[0], even_pixel[1], even_pixel[2], even_pixel[3]);

            if y + 1 < img.height() {
                let odd_pixel = img.get_pixel(x, y + 1);
                odds[y as usize / 2][x as usize] = (odd_pixel[0], odd_pixel[1], odd_pixel[2], odd_pixel[3]);
            }
        }
    }

    Frame::new(odds, evens)
}

