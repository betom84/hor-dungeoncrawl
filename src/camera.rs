use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(center: Point) -> Self {
        Self {
            left_x: center.x - DISPLAY_WIDTH / 2,
            right_x: center.x + DISPLAY_WIDTH / 2,
            top_y: center.y - DISPLAY_HEIGHT / 2,
            bottom_y: center.y + DISPLAY_HEIGHT / 2,
        }
    }

    pub fn on_move(&mut self, center: Point) {
        self.left_x = center.x - DISPLAY_WIDTH / 2;
        self.right_x = center.x + DISPLAY_WIDTH / 2;
        self.top_y = center.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = center.y + DISPLAY_HEIGHT / 2;
    }
}
