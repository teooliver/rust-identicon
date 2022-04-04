use md5;

#[derive(Debug)]
pub struct Identicon {
    pub hex: Vec<u8>,
    pub color: Vec<String>,
    pub grid: Vec<u8>,
    pub pixel_map: Vec<String>,
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
        self.color = vec![
            self.hex[0].to_string(),
            self.hex[1].to_string(),
            self.hex[2].to_string(),
        ]
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
            println!("{:?}", mirror_row(chunk.clone()));
            grid.push(mirror_row(chunk));
        }

        let flatened = grid.into_iter().flatten().collect();
        println!("{:?}", flatened);

        self.grid = flatened;
    }
}
