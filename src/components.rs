use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct BlocksTile;

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

impl Renderable {
    pub fn new(glyph: char, fg: (u8, u8, u8), bg: (u8, u8, u8)) -> Self {
        Self {
            glyph: to_cp437(glyph),
            fg: RGB::named(fg),
            bg: RGB::named(bg),
        }
    }
}

#[derive(Component)]
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

#[derive(Component)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub power: i32,
    pub defense: i32,
}

#[derive(Component)]
pub struct WantsToMelee {
    pub target: Entity,
}


#[derive(Component)]
pub struct SufferDamage {
    pub amount: Vec<i32>
}

impl SufferDamage {
    pub fn new() {
        
    }
}