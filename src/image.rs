use std::fs::File;
use std::io::Write;

use crate::vec3::Vec3;

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Vec3>,
}

impl Image {
    pub fn create(width: usize, height: usize) -> Self {
        Image::create_with_colour(width, height, 1.)
    }

    pub fn create_with_colour(width: usize, height: usize, default_colour: f32) -> Self {
        Image {
            width,
            height,
            data: vec![default_colour; 3 * width * height],
        }
    }

    pub fn save(&mut self, filepath: &str) -> Result<(), &str> {
        if filepath.ends_with(".ppm") {
            self.save_as_ppm(filepath);
        } else if filepath.ends_with(".bpm") {
            self.save_as_bmp(filepath)
        } else {
            return Err("Unknown filetype");
        }

        Ok(())
    }

    pub fn save_as_ppm(&mut self, filepath: &str) {
        let mut file = File::create(filepath).expect("Couldn't open file");

        let header = format!("P6 { } { } 255\n", self.width, self.height);
        file.write_all(header.as_bytes())
            .expect("Couldn't write header");

        let to_be_written = self.data
            .iter()
            .map(|e| ((e * 255.) as u8))
            .collect::<Vec<_>>();

        file.write_all(&to_be_written).expect("Couldn't write data");
    }

    pub fn save_as_bmp(&mut self, filepath: &str) {
        let mut file = File::create(filepath).expect("Couldn't open file");


    }

    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: Vec3) {
        let idx = (x + y * self.width) * 3;
        self.data[idx] = rgb.r();
        self.data[idx + 1] = rgb.g();
        self.data[idx + 2] = rgb.b();
    }
}
