use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, imageops::FilterType};
use std::{thread, time::Duration};
use termion::{terminal_size, clear, cursor};

struct Screen {
    x: u16,
    y: u16,
    buffer: Vec<Vec<char>>,
}

impl Screen {
    fn new() -> Self {
        let (x, y) = terminal_size().unwrap();
        let buffer = vec![vec![' '; x as usize]; y as usize];
        Screen { x, y, buffer }
    }

    fn img2ascii(&self, img: &DynamicImage) -> Vec<Vec<char>> {
        let img = img.to_luma8();
        let (width, height) = img.dimensions();
        let mut char_array = vec![vec![' '; width as usize]; height as usize];

        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y).0[0];
                let ch = match pixel {
                    0..=99 => ' ',
                    100..=199 => '*',
                    _ => '#',
                };
                char_array[y as usize][x as usize] = ch;
            }
        }
        char_array
    }

    fn render_sprite(&mut self, file: &str, size: (u32, u32), location: (u16, u16)) {
        let img = image::open(file).unwrap();
        let resized_img = img.resize_exact(size.0, size.1, FilterType::Nearest);
        let sprite_ascii = self.img2ascii(&resized_img);

        // Ensure the sprite fits within the screen bounds
        let (max_x, max_y) = (self.x as usize, self.y as usize);
        let (sprite_width, sprite_height) = (size.0 as usize, size.1 as usize);
        let (start_x, start_y) = (location.0 as usize, location.1 as usize);

        assert!(start_x + sprite_width <= max_x);
        assert!(start_y + sprite_height <= max_y);

        for y in 0..sprite_height {
            for x in 0..sprite_width {
                self.buffer[start_y + y][start_x + x] = sprite_ascii[y][x];
            }
        }
    }

    fn update(&self) {
        print!("{}", clear::All);
        for row in &self.buffer {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

fn process_frame() {
    thread::sleep(Duration::from_millis(10));
}

fn main() {
    let mut scr = Screen::new();
    let csx = scr.x / 9 + 1;
    let csy = scr.y / 3 + 1;

    scr.render_sprite("../sprites/f.jpg", (csx as u32, csy as u32), (0 * csx, 0 * csy));
    scr.render_sprite("../sprites/p.jpg", (csx as u32, csy as u32), (1 * csx, 0 * csy));
    scr.render_sprite("../sprites/s.jpg", (csx as u32, csy as u32), (2 * csx, 0 * csy));
    scr.render_sprite("../sprites/0.jpg", (csx as u32, csy as u32), (4 * csx, 0 * csy));
    scr.update();

    loop {
        let t0 = std::time::Instant::now();
        process_frame();
        let t1 = std::time::Instant::now();
        
        let elapsed = t1.duration_since(t0);
        let fps = 1.0 / elapsed.as_secs_f64();
        let fps = fps.round() as u32;

        let listed = fps.to_string();
        for (i, ch) in listed.chars().enumerate() {
            scr.render_sprite(&format!("../sprites/{}.jpg", ch), (csx as u32, csy as u32), ((i as u16 + 4) * csx, 0 * csy));
        }
        scr.update();
    }
}
