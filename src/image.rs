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
        let default_colour_rgb = Vec3::new(default_colour, default_colour, default_colour);

        Image {
            width,
            height,
            data: vec![default_colour_rgb; width * height],
        }
    }

    pub fn save(&mut self, filepath: &str) -> Result<(), &str> {
        if filepath.ends_with(".ppm") {
            self.save_as_ppm(filepath);
        } else if filepath.ends_with(".bmp") {
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

        let buff: &[u8] = &self
            .data
            .iter()
            .map(|pixel_array| (*pixel_array) * 255.)
            .flat_map(|pixel| [pixel.r() as u8, pixel.g() as u8, pixel.b() as u8])
            .collect::<Vec<_>>();

        file.write_all(buff).expect("Couldn't write data");
    }

    pub fn save_as_bmp(&mut self, filepath: &str) {
        let mut file = File::create(filepath).expect("Couldn't open file");

        let data: Vec<Vec3> = (0..self.height)
            .map(|y| {
                let offset = y * self.width;
                self.data[offset..offset+self.width].to_vec()
            })
            .rev()
            .flatten()
            .collect();

        let mut buff: Vec<u8> = data
            .iter()
            .map(|pixel_array| (*pixel_array) * 255.)
            .flat_map(|pixel| [pixel.b() as u8, pixel.g() as u8, pixel.r() as u8])
            .collect();

        (1..=self.height)
            .map(|y| y * 3 * self.width)
            .rev()
            .for_each(|i| {buff.insert(i, 00); buff.insert(i, 00)});

        let width: Vec<u8> = (self.width as u32).to_le_bytes().to_vec();
        let height: Vec<u8> = (self.height as u32).to_le_bytes().to_vec();

        let buff_size = ((self.data.len()*4) as u32).to_le_bytes();

        let mut info_header: Vec<u8> = vec![
            0x28, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00,
            0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        info_header.splice(4..8, width);
        info_header.splice(8..12, height);
        info_header.splice(20..24, buff_size);

        let mut header: Vec<u8> = vec![
            0x42, 0x4D, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00,
        ];

        let total_size: Vec<u8> = ((buff.len() + info_header.len() + 16) as u32)
            .to_le_bytes()
            .to_vec();

        header.splice(2..6, total_size);

        let mut result: Vec<u8> = Vec::new();

        result.append(&mut header);
        result.append(&mut info_header);
        result.append(&mut buff);

        file.write_all(&result).expect("Couldn't write data");
    }

    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: Vec3) {
        self.data[x + y * self.width] = rgb;
    }
}
