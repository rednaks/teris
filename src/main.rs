use macroquad::{miniquad::conf::Platform, prelude::*};

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct WorldLimits {
    pub left: f32,
    pub right: f32,
}

pub struct World {
    pub current_block: Position,
    pub blocks: Vec<Position>,
}

const SQUARE_SIZE: f32 = 60.0;
const STEP_DOWN: f32 = 1.0;
const STEP_HOR: f32 = 60.0;

const WINDOW_HEIGHT: i32 = 600;
const WINDOW_WIDTH: i32 = 800;

fn draw_grid(world_limits: &WorldLimits) {
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

fn draw_world_limits(world_limits: &WorldLimits) {
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

fn draw_world(world: &World) {
    draw_rectangle(
        world.current_block.x - SQUARE_SIZE / 2.0,
        world.current_block.y,
        SQUARE_SIZE,
        SQUARE_SIZE,
        GREEN,
    );
    for block in world.blocks.iter() {
        draw_rectangle(
            block.x - SQUARE_SIZE / 2.0,
            block.y,
            SQUARE_SIZE,
            SQUARE_SIZE,
            BLUE,
        );
    }
}

fn collide_with_other_block(world: &World) -> bool {
    for block in world.blocks.iter() {
        if world.current_block.y + SQUARE_SIZE == block.y && block.x == world.current_block.x {
            return true;
        }
    }
    false
}

fn update_world(world: &mut World, world_limits: &WorldLimits) {
    if world.current_block.y + SQUARE_SIZE > WINDOW_HEIGHT as f32
        || collide_with_other_block(&world)
    {
        world.blocks.push(world.current_block.clone());
        world.current_block = Position {
            x: (world_limits.left + world_limits.right) / 2.0,
            y: 0.0,
        };
    } else {
        world.current_block.y += STEP_DOWN;
    }

    if is_key_pressed(KeyCode::Left)
        && world.current_block.x - SQUARE_SIZE / 2.0 > world_limits.left
    {
        world.current_block.x -= STEP_HOR;
    } else if is_key_pressed(KeyCode::Right)
        && world.current_block.x + SQUARE_SIZE / 2.0 < world_limits.right
    {
        world.current_block.x += STEP_HOR;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Teris".to_owned(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen_center_x = WINDOW_WIDTH as f32 / 2.0;

    let world_limits = WorldLimits {
        left: (screen_center_x - SQUARE_SIZE / 2.0) - 5.0 * SQUARE_SIZE,
        right: (screen_center_x + SQUARE_SIZE / 2.0) + 5.0 * SQUARE_SIZE,
    };

    let mut world = World {
        blocks: Vec::new(),
        current_block: Position {
            x: screen_center_x,
            y: 0.0,
        },
    };

    loop {
        clear_background(BLACK);

        draw_grid(&world_limits);
        draw_world_limits(&world_limits);

        draw_world(&world);

        update_world(&mut world, &world_limits);

        next_frame().await
    }
}
