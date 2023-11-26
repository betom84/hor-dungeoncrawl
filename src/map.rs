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

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);

        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                let tile_idx = map_idx(x, y);
                if tile_idx.is_none() {
                    continue;
                }

                if let Some(tile) = self.tiles.get(tile_idx.expect("can't be none")) {
                    let x = x - camera.left_x;
                    let y = y - camera.top_y;

                    match tile { 
                    TileType::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                }
                }
            }
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
