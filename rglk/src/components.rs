use bevy::prelude::*;
use bevy_ascii_terminal::{FormattedTile, GridPoint, Pivot};
use bracket_geometry::prelude::Point;

#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub Point);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self(Point { x, y })
    }
}

impl GridPoint for Position {
    fn x(&self) -> i32 {
        self.0.x
    }

    fn y(&self) -> i32 {
        self.0.y
    }

    fn get_pivot(self) -> Option<Pivot> {
        None
    }
}

#[derive(Component, Debug, Clone)]
pub struct Renderable(pub FormattedTile);

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub dirty: bool,
    pub range: i32,
}

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: Vec::new(),
            dirty: true,
            range,
        }
    }
}
