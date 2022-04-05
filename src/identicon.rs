use std::vec;

use image::{ImageBuffer, Rgb, RgbImage};
use md5;

#[derive(Debug, Clone)]
pub struct Identicon {
    pub hex: Vec<u8>,
    pub color: [u8; 3],
    pub grid: Vec<(u8, u8)>,
    pub pixel_map: Vec<Square>,
}

#[derive(Debug, Clone)]
pub struct Square {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug, Clone)]
pub struct Point {
    x: u8,
    y: u8,
}

impl Identicon {
    pub fn hash_input(&mut self, input: &str) {
        let digest = md5::compute(input);
        let hex = digest.as_slice().to_owned();
        self.hex = hex.clone();
    }

    pub fn pick_color(&mut self) {
        self.color = [self.hex[0], self.hex[1], self.hex[2]]
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

    pub fn build_pixel_map(&mut self) {
        let mut pixel_map: Vec<Square> = vec![];
        for item in self.grid.clone() {
            // let mut index = 0;
            let horizontal = (item.1 % 5) * 50;
            let vertical = (item.1 / 5) * 50;

            let top_left = Point {
                x: horizontal,
                y: vertical,
            };
            let bottom_right = Point {
                x: horizontal + 50,
                y: vertical + 50,
            };

            pixel_map.push(Square {
                top_left,
                bottom_right,
            });
        }

        self.pixel_map = pixel_map;
    }

    pub fn paint_pixels(&mut self) {
        let imgx = 250;
        let imgy = 250;

        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

        for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
            *pixel = image::Rgb([250 as u8, 250 as u8, 250 as u8]);
        }
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            for square in self.pixel_map.clone() {
                if x >= square.top_left.x.into()
                    && x < square.bottom_right.x.into()
                    && y >= square.top_left.y.into()
                    && y < square.bottom_right.y.into()
                {
                    *pixel = image::Rgb(self.color);
                }
            }
        }

        // Save the image as “identicon.png”, the format is deduced from the path
        imgbuf.save("identicon.png").unwrap();
    }
}
