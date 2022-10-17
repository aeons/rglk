use bevy::prelude::*;
use bracket_bevy::prelude::*;
use bracket_bevy::FontCharType;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Monster;

#[derive(Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn as_point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub delta_x: i32,
    pub delta_y: i32,
}

#[derive(Component, Debug)]
pub struct BlocksTile;

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

impl Renderable {
    pub fn new(glyph: char, fg: Color, bg: Color) -> Self {
        Self {
            glyph: to_cp437(glyph),
            fg: fg.into(),
            bg: bg.into(),
        }
    }
}

#[derive(Component, Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
}

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: Vec::new(),
            range,
        }
    }
}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub power: i32,
    pub defense: i32,
}

#[derive(Component, Debug)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

#[derive(Component, Debug)]
pub struct Item;

#[derive(Component, Debug)]
pub struct Potion {
    pub heal_amount: i32,
}

#[derive(Component, Debug)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}
