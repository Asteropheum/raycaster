#[derive(Debug, Clone)]
pub struct FrameBuffer {
    pub width: i32,
    pub height: i32,
    pub image: Vec<u32>,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32, image: Vec<u32>) -> Box<Self> {
        Box::new(Self {
            width,
            height,
            image,
        })
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        assert!(
            self.image.len() == (self.width * self.height) as usize
                && x < self.width
                && y < self.height
        );
        self.image[(x + y * self.width) as usize] = color;
    }

    pub fn draw_rectangle(
        &mut self,
        rect_x: i32,
        rect_y: i32,
        rect_w: i32,
        rect_h: i32,
        color: u32,
    ) {
        assert_eq!(self.image.len(), (self.width * self.height) as usize);
        for i in 0..rect_w {
            for j in 0..rect_h {
                let cx = rect_x + i;
                let cy = rect_y + j;
                if (cx < self.width) && (cy < self.height) {
                    self.set_pixel(cx, cy, color);
                }
            }
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.image = vec![color; (self.width * self.height) as usize];
    }
}
