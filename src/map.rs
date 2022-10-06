use std::cmp::{max, min};

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn new_rooms_and_corridors() -> Self {
        let mut map = Self {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            revealed_tiles: vec![false; 80 * 50],
            visible_tiles: vec![false; 80 * 50],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.range(0, 80 - w - 2);
            let y = rng.range(0, 50 - h - 2);

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

    fn add_room(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                self.tiles[xy_idx(x, y)] = TileType::Floor;
            }
        }
    }

    fn add_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = xy_idx(x, y);
            if idx > 0 && idx < 80 * 50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn add_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = xy_idx(x, y);
            if idx > 0 && idx < 80 * 50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    pub fn reveal_tile(&mut self, x: i32, y: i32) {
        let idx = xy_idx(x, y);
        self.revealed_tiles[idx] = true;
        self.visible_tiles[idx] = true;
    }

    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        self.revealed_tiles[xy_idx(x, y)]
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.tiles[xy_idx(x, y)] != TileType::Wall
    }
}

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

pub fn draw_map(world: &mut World, ctx: &mut BTerm) {
    let map = world.resource::<Map>();
    let mut x = 0;
    let mut y = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let (glyph, mut fg) = match tile {
                TileType::Floor => (to_cp437('.'), RGB::named(TEAL)),
                TileType::Wall => (to_cp437('#'), RGB::named(GREEN)),
            };

            if !map.visible_tiles[idx] {
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
