use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb.place_amulet(mb.player_start);
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
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

    fn place_amulet(&mut self, player_start: Point) {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[self.map.point2d_to_index(player_start)],
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

        self.amulet_start = self.map.index_to_point2d(farthest_idx);
    }
}
