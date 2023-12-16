use crate::prelude::*;

use self::{
    automata::CellularAutomataArchitect,
    drunkard::DrunkardsWalkArchitect,
    prefab::apply_prefab,
    rooms::RoomsArchitect,
    themes::{DungeonTheme, ForestTheme},
};

mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;
mod themes;

trait MapArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(DrunkardsWalkArchitect {}),
            1 => Box::new(RoomsArchitect {}),
            _ => Box::new(CellularAutomataArchitect {}),
        };
        let mut mb = architect.build(rng);
        apply_prefab(&mut mb, rng);

        mb.theme = match rng.range(0, 2) {
            0 => DungeonTheme::new(),
            _ => ForestTheme::new(),
        };

        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024f32,
        );

        const UNREACHABLE: &f32 = &f32::MAX;
        let farthest_idx = dijkstra_map
            .map
            .iter()
            .enumerate()
            .filter(|(_, dist)| *dist < UNREACHABLE)
            .max_by(|(_, dist_a), (_, dist_b)| dist_a.partial_cmp(dist_b).unwrap())
            .unwrap()
            .0;

        self.map.index_to_point2d(farthest_idx)
    }

    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;

        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns: Vec<Point> = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let idx = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles.remove(idx))
        }

        spawns
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        'rooms: while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            for r in &self.rooms {
                if r.intersect(&room) {
                    continue 'rooms;
                }
            }

            room.for_each(|p| {
                if let Some(idx) = map_idx(p.x, p.y) {
                    self.map.tiles[idx] = TileType::Floor;
                }
            });

            self.rooms.push(room);
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let from = rooms[i - 1].center();
            let to = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(from.x, to.x, from.y);
                self.apply_vertical_tunnel(from.y, to.y, to.x);
            } else {
                self.apply_vertical_tunnel(from.y, to.y, from.x);
                self.apply_horizontal_tunnel(from.x, to.x, to.y);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};

        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = map_idx(x, y) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};

        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = map_idx(x, y) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }
}
