use std::vec;

use image;
use md5;

#[derive(Debug, Clone)]
pub struct Identicon {
    pub hex: Vec<u8>,
    pub color: [u8; 3], // TODO: Refactor to tuple? Or maybe a struct? RBG{r:0,g:0,b:0}
    pub grid: Vec<(u8, u8)>,
    pub pixel_map: Vec<Square>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Square {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    x: u8,
    y: u8,
}

impl Identicon {
    pub fn new() -> Self {
        Identicon {
            hex: vec![],
            color: [0, 0, 0],
            grid: vec![],
            pixel_map: vec![],
        }
    }

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
            if chunk.len() != 3 {
                break;
            }
            grid.push(mirror_row(chunk));
        }

        let flatened: Vec<u8> = grid.into_iter().flatten().collect();

        let mut flat_with_index: Vec<(u8, u8)> = vec![];

        let mut index: u8 = 0;
        for item in flatened {
            flat_with_index.push((item, index));
            index += 1;
        }

        self.grid = flat_with_index;
    }

    pub fn remove_odd_squares(&mut self) {
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

// Extract to __tests__ folder
#[cfg(test)]
mod tests {
    use crate::identicon::{Identicon, Point, Square};

    #[test]
    fn hash_input_impl() {
        let mut identicon = Identicon {
            hex: vec![],
            color: [0, 0, 0],
            grid: vec![],
            pixel_map: vec![],
        };

        identicon.hash_input("banana");

        assert_eq!(
            identicon.hex,
            [114, 179, 2, 191, 41, 122, 34, 138, 117, 115, 1, 35, 239, 239, 124, 65]
        );
    }

    #[test]
    fn pick_color_impl() {
        let mut identicon = Identicon {
            hex: vec![
                114, 179, 2, 191, 41, 122, 34, 138, 117, 115, 1, 35, 239, 239, 124, 65,
            ],
            color: [0, 0, 0],
            grid: vec![],
            pixel_map: vec![],
        };

        identicon.pick_color();

        assert_eq!(identicon.color, [114, 179, 2]);
    }
    #[test]
    fn build_grid_impl() {
        let mut identicon = Identicon {
            hex: vec![
                114, 179, 2, 191, 41, 122, 34, 138, 117, 115, 1, 35, 239, 239, 124, 65,
            ],
            color: [114, 179, 2],
            grid: vec![],
            pixel_map: vec![],
        };

        identicon.build_grid();

        assert_eq!(
            identicon.grid,
            [
                (114, 0),
                (179, 1),
                (2, 2),
                (179, 3),
                (114, 4),
                (191, 5),
                (41, 6),
                (122, 7),
                (41, 8),
                (191, 9),
                (34, 10),
                (138, 11),
                (117, 12),
                (138, 13),
                (34, 14),
                (115, 15),
                (1, 16),
                (35, 17),
                (1, 18),
                (115, 19),
                (239, 20),
                (239, 21),
                (124, 22),
                (239, 23),
                (239, 24)
            ]
        );
    }
    #[test]
    fn filter_odd_squares_impl() {
        let mut identicon = Identicon {
            hex: vec![
                114, 179, 2, 191, 41, 122, 34, 138, 117, 115, 1, 35, 239, 239, 124, 65,
            ],
            color: [114, 179, 2],
            grid: vec![
                (114, 0),
                (179, 1),
                (2, 2),
                (179, 3),
                (114, 4),
                (191, 5),
                (41, 6),
                (122, 7),
                (41, 8),
                (191, 9),
                (34, 10),
                (138, 11),
                (117, 12),
                (138, 13),
                (34, 14),
                (115, 15),
                (1, 16),
                (35, 17),
                (1, 18),
                (115, 19),
                (239, 20),
                (239, 21),
                (124, 22),
                (239, 23),
                (239, 24),
            ],
            pixel_map: vec![],
        };

        identicon.remove_odd_squares();

        assert_eq!(
            identicon.grid,
            [
                (114, 0),
                (2, 2),
                (114, 4),
                (122, 7),
                (34, 10),
                (138, 11),
                (138, 13),
                (34, 14),
                (124, 22)
            ]
        );
    }
    #[test]
    fn build_pixel_map_impl() {
        let mut identicon = Identicon {
            hex: vec![
                114, 179, 2, 191, 41, 122, 34, 138, 117, 115, 1, 35, 239, 239, 124, 65,
            ],
            color: [114, 179, 2],
            grid: vec![
                (114, 1),
                (2, 3),
                (114, 5),
                (122, 8),
                (34, 11),
                (138, 12),
                (138, 14),
                (34, 15),
                (124, 23),
            ],
            pixel_map: vec![],
        };

        identicon.build_pixel_map();

        assert_eq!(
            identicon.pixel_map,
            [
                Square {
                    top_left: Point { x: 50, y: 0 },
                    bottom_right: Point { x: 100, y: 50 }
                },
                Square {
                    top_left: Point { x: 150, y: 0 },
                    bottom_right: Point { x: 200, y: 50 }
                },
                Square {
                    top_left: Point { x: 0, y: 50 },
                    bottom_right: Point { x: 50, y: 100 }
                },
                Square {
                    top_left: Point { x: 150, y: 50 },
                    bottom_right: Point { x: 200, y: 100 }
                },
                Square {
                    top_left: Point { x: 50, y: 100 },
                    bottom_right: Point { x: 100, y: 150 }
                },
                Square {
                    top_left: Point { x: 100, y: 100 },
                    bottom_right: Point { x: 150, y: 150 }
                },
                Square {
                    top_left: Point { x: 200, y: 100 },
                    bottom_right: Point { x: 250, y: 150 }
                },
                Square {
                    top_left: Point { x: 0, y: 150 },
                    bottom_right: Point { x: 50, y: 200 }
                },
                Square {
                    top_left: Point { x: 150, y: 200 },
                    bottom_right: Point { x: 200, y: 250 }
                }
            ]
        );
    }
}
