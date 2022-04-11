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

        let data = (0..self.height)
            .map(|y| {
                let offset = y * self.width;
                self.data[offset..offset + self.width].to_vec()
            })
            .rev()
            .flatten();

        let mut buff: Vec<u8> = data
            .map(|pixel_array| pixel_array * 255.)
            .flat_map(|pixel| [pixel.b() as u8, pixel.g() as u8, pixel.r() as u8])
            .collect();

        let padding_multiplyer = self.width % 4;

        if padding_multiplyer != 0 {
            (1..=self.height)
                .map(|y| y * 3 * self.width)
                .rev()
                .for_each(|i| {
                    for _ in 0..padding_multiplyer {
                        buff.insert(i, 00)
                    }
                });
        }

        let mut info_header: [u8; 40] = [
            0x28, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00,
            0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        info_header[4..8].copy_from_slice(&(self.width as u32).to_le_bytes());
        info_header[8..12].copy_from_slice(&(self.height as u32).to_le_bytes());
        info_header[20..24].copy_from_slice(&(((self.data.len() * 4) as u32).to_le_bytes()));

        let mut header: [u8; 14] = [
            0x42, 0x4D, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00,
        ];

        let total_size: [u8; 4] = ((buff.len() + info_header.len() + 16) as u32)
            .to_le_bytes();

        header[2..6].copy_from_slice(&total_size);

        let result: Vec<u8> = [header.to_vec(), info_header.to_vec(), buff]
            .iter()
            .flatten()
            .copied()
            .collect();

        file.write_all(&result).expect("Couldn't write data");
    }

    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: Vec3) {
        self.data[x + y * self.width] = rgb;
    }
}
