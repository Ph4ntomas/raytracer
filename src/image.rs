use crate::Dimension;

pub struct Image{
    pub x: u32,
    pub y: u32,
    data: Vec<u8>
}

impl Image {
    pub fn new(x: u32, y: u32, color: Option<u32>) -> Self {
        let mut ret = Image {
            x,
            y,
            data: Vec::with_capacity((x * y * 4) as usize)
        };

        ret.data.resize((4 * x * y) as usize, 0);

        if let Some(col) = color {
            for a in 0..x {
                for b in 0..y {
                    ret.draw_pixel(a, b, col);
                }
            }
        }

        ret
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        let index : usize = (x * 4 * self.y + 4 * y) as usize;

        self.data[index] = color.to_be_bytes()[1];
        self.data[index + 1] = color.to_be_bytes()[2];
        self.data[index + 2] = color.to_be_bytes()[3];
        self.data[index + 3] = color.to_be_bytes()[0];
    }

    pub fn get_dimension(&self) -> Dimension {
        Dimension { x: self.x, y: self.y }
    }

    pub fn get_data(&self) -> &[u8] {
        self.data.as_slice()
    }
}
