use macroquad::prelude::*;

use structs::*;
pub mod structs; 

const SQUARE_SIZE: f32 = 60.0;
const STEP_DOWN: f32 = 1.0;
const STEP_HOR: f32 = 60.0;

const WINDOW_HEIGHT: i32 = 600;
const WINDOW_WIDTH: i32 = 800;

pub fn draw_grid(world_limits: &WorldLimits) {
    // draw horizontal lines
    let horizontal_lines = WINDOW_HEIGHT as f32 / SQUARE_SIZE;

    for line_number in 0..horizontal_lines as i32 {
        let y = 0.0 + line_number as f32 * SQUARE_SIZE;
        draw_line(world_limits.left, y, world_limits.right, y, 1.0, RED);
    }

    // draw vertical lines
    let vertical_lines = WINDOW_WIDTH as f32 / SQUARE_SIZE;
    for line_number in 0..(vertical_lines as i32) - 1 {
        let x = world_limits.left + line_number as f32 * SQUARE_SIZE;
        draw_line(x, 0.0, x, WINDOW_HEIGHT as f32, 1.0, RED);
    }
}

pub fn draw_world_limits(world_limits: &WorldLimits) {
    draw_line(
        world_limits.left,
        0.0,
        world_limits.left,
        WINDOW_HEIGHT as f32,
        1.0,
        WHITE,
    );

    draw_line(
        world_limits.right,
        0.0,
        world_limits.right,
        WINDOW_HEIGHT as f32,
        1.0,
        WHITE,
    );
}

pub fn draw_world(world: &World) {
    draw_rectangle(
        world.current_block.position.x - SQUARE_SIZE / 2.0,
        world.current_block.position.y,
        SQUARE_SIZE,
        SQUARE_SIZE,
        GREEN,
    );
    for block in world.blocks.iter() {
        draw_rectangle(
            block.position.x - SQUARE_SIZE / 2.0,
            block.position.y,
            SQUARE_SIZE,
            SQUARE_SIZE,
            BLUE,
        );
    }
}

pub fn collide_with_other_block(world: &World) -> bool {
    for block in world.blocks.iter() {
        if block.position.x == world.current_block.position.x {
            if let Some(_) = world.current_block.rect.intersect(block.rect) {
                return true;
            }
        }
    }
    false
}

pub fn check_new_block_position_is_filled(world: &World) -> bool {
    for block in world.blocks.iter() {
        if block.position.x == world.current_block.position.x
            && block.position.y == world.current_block.position.y
        {
            return true;
        }
    }
    false
}

pub fn left_block_detected(world: &World) -> bool {
    for block in world.blocks.iter() {
        if world.current_block.position.x - SQUARE_SIZE == block.position.x {
            if let Some(_) = world.current_block.rect.intersect(block.rect) {
                return true;
            }
        }
    }
    return false;
}

pub fn right_block_detected(world: &World) -> bool {
    for block in world.blocks.iter() {
        if world.current_block.position.x + SQUARE_SIZE == block.position.x {
            if let Some(_) = world.current_block.rect.intersect(block.rect) {
                return true;
            }
        }
    }
    return false;
}

pub fn update_world(world: &mut World, world_limits: &WorldLimits) -> bool {
    if world.current_block.position.y + SQUARE_SIZE > WINDOW_HEIGHT as f32
        || collide_with_other_block(&world)
    {
        world.blocks.push(world.current_block.clone());
        world.current_block = Block {
            position: Position {
                x: (world_limits.left + world_limits.right) / 2.0,
                y: 0.0,
            },
            rect: Rect {
                x: (world_limits.left + world_limits.right) / 2.0,
                y: 0.0,
                w: SQUARE_SIZE,
                h: SQUARE_SIZE,
            },
        };
        if check_new_block_position_is_filled(&world) {
            return true;
        }
    } else if is_key_down(KeyCode::Down){
        world.current_block.position.y += STEP_DOWN * 5.0;
        world.current_block.rect.y += STEP_DOWN * 5.0;
    } else {
        world.current_block.position.y += STEP_DOWN;
        world.current_block.rect.y += STEP_DOWN;
    }

    if is_key_pressed(KeyCode::Left)
        && !left_block_detected(&world)
        && world.current_block.position.x - SQUARE_SIZE / 2.0 > world_limits.left
    {
        world.current_block.position.x -= STEP_HOR;
        world.current_block.rect.x -= STEP_HOR;
    } else if is_key_pressed(KeyCode::Right)
        && !right_block_detected(&world)
        && world.current_block.position.x + SQUARE_SIZE / 2.0 < world_limits.right
    {
        world.current_block.position.x += STEP_HOR;
        world.current_block.rect.x += STEP_HOR;
    } 

    return false;
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Teris".to_owned(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}