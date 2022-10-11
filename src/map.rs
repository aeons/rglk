use std::cmp::{max, min};

use bevy::prelude::*;
use bit_set::BitSet;
use bracket_bevy::prelude::*;
use bracket_pathfinding::prelude::*;

use crate::rect::Rect;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: BitSet,
    pub visible_tiles: BitSet,
    pub blocked_tiles: BitSet,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new_rooms_and_corridors(rng: &RandomNumbers) -> Self {
        let mut map = Self {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            revealed_tiles: BitSet::new(),
            visible_tiles: BitSet::new(),
            blocked_tiles: BitSet::new(),
            tile_content: vec![Vec::new(); 80 * 50],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.range(0, map.width - w - 2);
            let y = rng.range(0, map.height - h - 2);

            let new_room = Rect::new(x, y, w, h);

            if !map.rooms.iter().any(|r| new_room.intersects(r)) {
                map.add_room(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map
                        .rooms
                        .last()
                        .expect("rooms should not be empty")
                        .center();

                    if rng.rand::<bool>() {
                        map.add_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.add_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.add_vertical_tunnel(prev_y, new_y, prev_x);
                        map.add_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room)
            }
        }

        map
    }

    pub fn draw(&self, ctx: &BracketContext) {
        let mut x = 0;
        let mut y = 0;

        for (idx, tile) in self.tiles.iter().enumerate() {
            if self.revealed_tiles.contains(idx) {
                let (glyph, mut fg) = match tile {
                    TileType::Floor => (to_cp437('.'), RGB::named(TEAL)),
                    TileType::Wall => (to_cp437('#'), RGB::named(GREEN)),
                };

                if !self.visible_tiles.contains(idx) {
                    fg = fg.to_greyscale()
                }

                ctx.set(x, y, fg, RGB::named(BLACK), glyph);
            }

            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.width, idx as i32 / self.width)
    }

    fn add_room(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn add_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.height * self.width) as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn add_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < (self.height * self.width) as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    pub fn populate_blocked(&mut self) {
        self.blocked_tiles.clear();
        for (idx, tile) in self.tiles.iter().enumerate() {
            if *tile == TileType::Wall {
                self.blocked_tiles.insert(idx);
            }
        }
    }

    pub fn reveal_tile(&mut self, x: i32, y: i32) {
        let idx = self.xy_idx(x, y);
        self.revealed_tiles.insert(idx);
        self.visible_tiles.insert(idx);
    }

    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        self.revealed_tiles.contains(self.xy_idx(x, y))
    }

    pub fn is_valid_exit(&self, x: i32, y: i32) -> bool {
        x > 0 && x < self.width && y > 0 && y < self.height && {
            let idx = self.xy_idx(x, y);
            !self.blocked_tiles.contains(idx)
        }
    }

    pub fn clear_content_index(&mut self) {
        for tile in self.tile_content.iter_mut() {
            tile.clear()
        }
    }
}

impl FromWorld for Map {
    fn from_world(world: &mut World) -> Self {
        let rng = world.resource::<RandomNumbers>();
        Self::new_rooms_and_corridors(rng)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let (x, y) = self.idx_xy(idx);
        let w = self.width as usize;

        // Cardinal directions
        if self.is_valid_exit(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_valid_exit(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_valid_exit(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_valid_exit(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        // Diagonals
        if self.is_valid_exit(x - 1, y - 1) {
            exits.push((idx - w - 1, 1.0))
        };
        if self.is_valid_exit(x + 1, y - 1) {
            exits.push((idx - w + 1, 1.0))
        };
        if self.is_valid_exit(x - 1, y + 1) {
            exits.push((idx + w - 1, 1.0))
        };
        if self.is_valid_exit(x + 1, y + 1) {
            exits.push((idx + w + 1, 1.0))
        };

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let (x1, y1) = self.idx_xy(idx1);
        let p1 = Point::new(x1, y1);
        let (x2, y2) = self.idx_xy(idx2);
        let p2 = Point::new(x2, y2);

        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
