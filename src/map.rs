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

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;

        if self.can_enter_tile(destination) {
            Some(self.point2d_to_index(destination))
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let loc = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(loc, Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }

        if let Some(idx) = self.valid_exit(loc, Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }

        if let Some(idx) = self.valid_exit(loc, Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }

        if let Some(idx) = self.valid_exit(loc, Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point {
            x: SCREEN_WIDTH,
            y: SCREEN_HEIGHT,
        }
    }

    fn in_bounds(&self, pos: Point) -> bool {
        in_bounds(pos)
    }
}

pub fn map_idx(x: i32, y: i32) -> Option<usize> {
    if in_bounds(Point::new(x, y)) {
        Some(((y * SCREEN_WIDTH) + x) as usize)
    } else {
        None
    }
}

pub fn in_bounds(p: Point) -> bool {
    p.x >= 0 && p.x < SCREEN_WIDTH && p.y >= 0 && p.y < SCREEN_HEIGHT
}
