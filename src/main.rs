use macroquad::prelude::*;
use macroquad::ui;

use teris::structs::*;


use teris::draw_grid;
use teris::*;

const SQUARE_SIZE: f32 = 60.0;

const WINDOW_HEIGHT: i32 = 600;
const WINDOW_WIDTH: i32 = 800;

#[macroquad::main(window_conf)]
async fn main() {
    let screen_center_x = WINDOW_WIDTH as f32 / 2.0;

    let our_world_limits = WorldLimits {
        left: (screen_center_x - SQUARE_SIZE / 2.0) - 5.0 * SQUARE_SIZE,
        right: (screen_center_x + SQUARE_SIZE / 2.0) + 5.0 * SQUARE_SIZE,
    };

    let mut our_world = World {
        blocks: Vec::new(),
        current_block: Block {
            position: Position {
                x: screen_center_x,
                y: 0.0,
            },
            rect: Rect {
                x: screen_center_x,
                y: 0.0,
                w: SQUARE_SIZE,
                h: SQUARE_SIZE,
            },
        },
    };

    let mut our_game_state = GameState {
        score: 0,
        game_over: false,
        paused: false,
    };
    loop {
        if ui::root_ui().button(None, "Pause") {
            our_game_state.paused = !our_game_state.paused;
        }

        if ui::root_ui().button(None, "Game Over") {
            our_game_state.game_over = !our_game_state.game_over;
            
        }
        if ui::root_ui().button(None, "Restart") {
            our_game_state.game_over = false;
            our_world.blocks = Vec::new();
            our_world.current_block = Block {
                position: Position {
                    x: screen_center_x,
                    y: 0.0,
                },
                rect: Rect {
                    x: screen_center_x,
                    y: 0.0,
                    w: SQUARE_SIZE,
                    h: SQUARE_SIZE,
                },
            };
        }

        clear_background(BLACK);
        draw_grid(&our_world_limits);
        draw_world_limits(&our_world_limits);
        draw_world(&our_world);

        if !(our_game_state.paused || our_game_state.game_over) {
            our_game_state.game_over = update_world(&mut our_world, &our_world_limits);
        }

        if our_game_state.game_over {
            draw_text(
                "Game Over!",
                screen_center_x - 120.0,
                WINDOW_HEIGHT as f32 / 2.0,
                60.0,
                WHITE,
            );
        }

        next_frame().await
    }
}
