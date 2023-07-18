use bracket_geometry::prelude::Rect;
use fixedbitset::FixedBitSet;

use crate::prelude::*;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;
pub const MAP_COUNT: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Debug, Resource)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub dimensions: Point,
    pub revealed_tiles: FixedBitSet,
    pub visible_tiles: FixedBitSet,
}

impl Map {
    pub fn new_rooms_and_corridors(rng: &mut Rng) -> Self {
        let mut map = Self {
            tiles: vec![TileType::Wall; MAP_COUNT],
            rooms: Vec::new(),
            dimensions: Point::new(MAP_WIDTH, MAP_HEIGHT),
            revealed_tiles: FixedBitSet::with_capacity(MAP_COUNT),
            visible_tiles: FixedBitSet::with_capacity(MAP_COUNT),
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        for _ in 0..MAX_ROOMS {
            let w = rng.i32(MIN_SIZE..MAX_SIZE);
            let h = rng.i32(MIN_SIZE..MAX_SIZE);
            let x = rng.i32(0..(map.dimensions.x - w - 2));
            let y = rng.i32(0..(map.dimensions.y - h - 2));

            let new_room = Rect::with_size(x, y, w, h);

            if !map.rooms.iter().any(|r| new_room.intersect(r)) {
                map.add_room(&new_room);

                if let Some(prev_center) = map.rooms.last().map(|r| r.center()) {
                    let new_center = new_room.center();

                    let corner = if rng.bool() {
                        Point::new(prev_center.x, new_center.y)
                    } else {
                        Point::new(new_center.x, prev_center.y)
                    };

                    map.add_tunnel(prev_center, corner);
                    map.add_tunnel(corner, new_center);
                }

                map.rooms.push(new_room)
            }
        }

        map
    }

    pub fn render(&self, term: &mut Terminal) {
        let mut x = 0;
        let mut y = 0;
        for tile in self.tiles.iter() {
            let point = self.point2d_to_index((x,y).into());
            if self.revealed_tiles[point] {
                let (glyph, mut fg) = match tile {
                    TileType::Floor => ('.', Color::GRAY),
                    TileType::Wall => ('#',Color::GREEN),
                };
                if !self.visible_tiles[point] { fg = to_greyscale(fg)}

                term.put_char([x, y], glyph.fg(fg));
            }

            x += 1;
            if x > MAP_WIDTH - 1 {
                x = 0;
                y += 1;
            }
        }
    }

    fn add_room(&mut self, room: &Rect) {
        room.for_each(|p| {
            let idx = self.point2d_to_index(p);
            self.tiles[idx] = TileType::Floor;
        });
    }

    fn add_tunnel(&mut self, p1: Point, p2: Point) {
        for p in Bresenham::new(p1, p2) {
            if self.in_bounds(p) {
                let idx = self.point2d_to_index(p);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
}

impl FromWorld for Map {
    fn from_world(world: &mut World) -> Self {
        let mut global_rng = world.resource_mut::<GlobalRng>();
        let rng = global_rng.get_mut();
        Map::new_rooms_and_corridors(rng)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        self.dimensions
    }
}

fn to_greyscale(color: Color) -> Color {
    let linear = (color.r() * 0.2126) + (color.g() * 0.7152) + (color.b() * 0.0722);
    Color::rgb(linear, linear, linear)
}
