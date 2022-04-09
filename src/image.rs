use std::fs::File;
use std::io::Write;
use std::str::Split;

use crate::vec3::Vec3;

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<f32>,
}

impl Image {
    pub fn create(width: usize, height: usize) -> Self {
        Image::create_with_colour(width, height, 1.)
    }

    pub fn create_with_colour(width: usize, height: usize, default_colour: f32) -> Self {
        Image { width, height, data: vec![default_colour; 3 * width * height] }
    }

    pub fn save(&mut self, filepath: &str) -> Result<(), &str> {
        if filepath.ends_with(".ppm") {
            self.save_as_ppm(filepath);
        } else {
            return Err("Unknown filetype");
        }

        return Ok(());
    }

    pub fn save_as_ppm(&mut self, filepath: &str) {
        let mut file = File::create(filepath)
            .expect("Couldn't open file");

        let header = format!(
            "P6 { } { } 255\n", self.width.to_string(), self.height.to_string());

        file.write(header.as_bytes())
            .expect("Couldn't write header");


        let mut to_be_written: Vec<u8> = Vec::new();

        self.data
            .iter()
            .for_each(|e| to_be_written.push((e * 255.) as u8));

        file.write(&to_be_written)
            .expect("Couldn't write data");
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: Vec3) {
        let idx = (x + y * self.width) * 3;
        self.data[idx + 0] = rgb.r();
        self.data[idx + 1] = rgb.g();
        self.data[idx + 2] = rgb.b();
    }
}
