use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn can_enter_tile(&self, p: Point) -> bool {
        if let Some(idx) = map_idx(p.x, p.y) {
            if let Some(&tile) = self.tiles.get(idx) {
                return tile == TileType::Floor;
            }
        }

        false
    }
}

pub fn map_idx( x: i32, y: i32) -> Option<usize> {
    if in_bounds(Point::new(x, y)) {
        Some(((y * SCREEN_WIDTH) + x) as usize)
    } else {
        None
    }
}

pub fn in_bounds(p: Point) -> bool {
    p.x >= 0 && p.x < SCREEN_WIDTH &&
    p.y >= 0 && p.y < SCREEN_HEIGHT
}
