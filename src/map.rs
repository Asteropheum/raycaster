pub struct Map {
    width: i32,
    height: i32,
    map: Vec<char>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            map: "0000222222220000\
                 1              0\
                 1      11111   0\
                 1     0        0\
                 0     0  1110000\
                 0     3        0\
                 0   10000      0\
                 0   3   11100  0\
                 5   4   0      0\
                 5   4   1  00000\
                 0       1      0\
                 2       1      0\
                 0       0      0\
                 0 0000000      0\
                 0              0\
                 0002222222200000"
                .chars()
                .collect(),
        }
    }

    pub fn get(&self, i: i32, j: i32) -> u32 {
        assert!(
            i < self.width
                && j < self.height
                && self.map.len() == (self.width * self.height) as usize
        );
        self.map[(i + j * self.width) as usize]
            .to_digit(10)
            .unwrap()
    }

    pub fn is_empty(&self, i: i32, j: i32) -> bool {
        assert!(
            i < self.width
                && j < self.height
                && self.map.len() == (self.width * self.height) as usize
        );
        self.map[(i + j * self.width) as usize] == ' '
    }
}
