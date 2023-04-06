use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub position: Position,
    pub rect: Rect,
}

pub struct WorldLimits {
    pub left: f32,
    pub right: f32,
}

pub struct World {
    pub current_block: Block,
    pub blocks: Vec<Block>,
}

pub struct GameState {
    pub score: i32,
    pub game_over: bool,
    pub paused: bool,
}