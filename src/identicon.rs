use std::vec;

use image::{Rgb, RgbImage};
use md5;

#[derive(Debug)]
pub struct Identicon {
    pub hex: Vec<u8>,
    pub color: Vec<u8>,
    pub grid: Vec<(u8, u8)>,
    pub pixel_map: Vec<((u8, u8), (u8, u8))>,
}

impl Identicon {
    pub fn hash_input(&mut self, input: &str) -> Vec<u8> {
        // create a Md5 hasher instance
        let digest = md5::compute(input);
        let hex = digest.as_slice().to_owned();
        self.hex = hex.clone();

        hex
    }

    pub fn pick_color(&mut self) {
        self.color = vec![self.hex[0], self.hex[1], self.hex[2]]
    }

    pub fn build_grid(&mut self) {
        pub fn mirror_row(row: Vec<u8>) -> Vec<u8> {
            if row.len() >= 3 {
                let mirrored_row = vec![row[0], row[1], row[2], row[1], row[0]];
                return mirrored_row;
            }

            row
        }
        let chunks: Vec<Vec<u8>> = self.hex.chunks(3).map(|x| x.to_vec()).collect();

        let mut grid: Vec<Vec<u8>> = vec![];

        for chunk in chunks {
            // println!("{:?}", mirror_row(chunk.clone()));
            grid.push(mirror_row(chunk));
        }

        let flatened: Vec<u8> = grid.into_iter().flatten().collect();

        let mut flat_with_index: Vec<(u8, u8)> = vec![];

        let mut index: u8 = 1;
        for item in flatened {
            flat_with_index.push((item, index));
            index += 1;
        }

        self.grid = flat_with_index;
        // println!("{:?}", flatened);
    }

    pub fn filter_odd_squares(&mut self) {
        let filtered: Vec<(u8, u8)> = self
            .grid
            .clone()
            .into_iter()
            .filter(|x| x.0 % 2 == 0)
            .collect();

        self.grid = filtered;
    }

    pub fn draw_image(&mut self) {
        let imgx = 250;
        let imgy = 250;

        let scalex = 3.0 / imgx as f32;
        let scaley = 3.0 / imgy as f32;

        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            // let r = (0.3 * x as f32) as u8;
            // let b = (0.3 * y as f32) as u8;
            // *pixel = image::Rgb([r, 0, b]);

            *pixel = image::Rgb([
                self.color[0] as u8,
                self.color[1].clone() as u8,
                self.color[2].clone() as u8,
            ]);
        }

        // A redundant loop to demonstrate reading image data
        for x in 0..imgx {
            for y in 0..imgy {
                let cx = y as f32 * scalex - 1.5;
                let cy = x as f32 * scaley - 1.5;

                let c = num_complex::Complex::new(-0.4, 0.6);
                let mut z = num_complex::Complex::new(cx, cy);

                let mut i = 0;
                while i < 255 && z.norm() <= 2.0 {
                    z = z * z + c;
                    i += 1;
                }

                let pixel = imgbuf.get_pixel_mut(x, y);
                let image::Rgb(data) = *pixel;
                *pixel = image::Rgb([data[0], i as u8, data[2]]);
            }
        }

        // Save the image as “fractal.png”, the format is deduced from the path
        imgbuf.save("identicon.png").unwrap();
    }

    pub fn build_pixel_map(&mut self) {
        let mut pixel_map: Vec<((u8, u8), (u8, u8))> = vec![];
        for item in self.grid.clone() {
            // let mut index = 0;
            let horizontal = (item.1 % 5) * 50;
            let vertical = (item.1 / 5) * 50;

            let top_left = (horizontal, vertical);
            let bottom_right = (horizontal + 50, vertical + 50);

            pixel_map.push((top_left, bottom_right));
        }

        self.pixel_map = pixel_map;
    }

    pub fn paint_pixels(&mut self) {
        for x in 0..250 {
            for y in 0..250 {
                // let cx = y as f32 * scalex - 1.5;
                // let cy = x as f32 * scaley - 1.5;

                // let c = num_complex::Complex::new(-0.4, 0.6);
                // let mut z = num_complex::Complex::new(cx, cy);

                // let mut i = 0;
                // while i < 255 && z.norm() <= 2.0 {
                //     z = z * z + c;
                //     i += 1;
                // }

                // let pixel = imgbuf.get_pixel_mut(x, y);
                // let image::Rgb(data) = *pixel;
                // *pixel = image::Rgb([data[0], i as u8, data[2]]);
            }
        }
    }
}

// [   X         Y
// ((50, 0), (100, 50)),
// ((150, 0), (200, 50)),
// ((0, 50), (50, 100)),
// ((150, 50), (200, 100)),
// ((50, 100), (100, 150)),
// ((100, 100), (150, 150)),
// ((200, 100), (250, 150)),
// ((0, 150), (50, 200)),
// ((150, 200), (200, 250))
// ]
