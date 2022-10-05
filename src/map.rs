use bracket_lib::prelude::*;

pub struct Map(Vec<TileType>);

impl Map {
    pub fn new() -> Self {
        let mut map = vec![TileType::Floor; 80 * 50];

        // Make the boundaries walls
        for x in 0..80 {
            map[xy_idx(x, 0)] = TileType::Wall;
            map[xy_idx(x, 49)] = TileType::Wall;
        }
        for y in 0..50 {
            map[xy_idx(0, y)] = TileType::Wall;
            map[xy_idx(79, y)] = TileType::Wall;
        }

        // Randomly add a bunch of walls
        let mut rng = RandomNumberGenerator::new();

        for _i in 0..400 {
            let x = rng.range(0, 80);
            let y = rng.range(0, 50);
            let idx = xy_idx(x, y);
            if idx != xy_idx(40, 25) {
                map[idx] = TileType::Wall;
            }
        }

        Self(map)
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        let mut x = 0;
        let mut y = 0;

        for tile in self.0.iter() {
            match tile {
                TileType::Floor => {
                    ctx.set(x, y, RGB::named(WEBGREY), RGB::named(BLACK), to_cp437('.'))
                }
                TileType::Wall => {
                    ctx.set(x, y, RGB::named(GREEN), RGB::named(BLACK), to_cp437('#'))
                }
            }

            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }
    }

    pub fn is_accessible(&self, x: i32, y: i32) -> bool {
        self.0[xy_idx(x, y)] != TileType::Wall
    }
}

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}
