use std::ops::Deref;

use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub Point);

impl Deref for Position {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.0
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
pub struct Monster;

#[derive(Component, Debug)]
pub struct BlocksTile;

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

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}
