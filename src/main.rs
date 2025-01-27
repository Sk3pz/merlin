use std::time::Instant;
use gamestate::GameState;
use macroquad::prelude::*;

pub mod logging;
pub mod gamestate;
pub mod controls;
pub mod player;
pub mod aircraft;

pub const DEBUG_OUTPUT: bool = true;
const FPS_SMOOTHING_FRAMES: usize = 30;

fn window_config() -> Conf {
    Conf {
        window_title: "Merlin".to_string(),
        window_width: 1200,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {

    debug!("Initializing assets...");

    // todo: handle assets here?

    debug!("Assets initialized.");
    debug!("Initializing gamestate...");
    
    // create a dynamic gamestate object
    let gamestate = gamestate::playing::PlayingGS::new().await;
    if let Err(e) = gamestate {
        return eprintln!("Failed to initialize gamestate: {}", e);
    }
    let mut gamestate: Box<dyn GameState> = gamestate.unwrap();

    debug!("Gamestate initialized.");
    debug!("Starting game loop...");
    
    // handle FPS calculations
    let mut last_time = Instant::now();
    let mut fps_values = vec![0.0; FPS_SMOOTHING_FRAMES];
    let mut fps_index = 0;
    let mut fps_sum = 0.0;

    'game_loop: loop { // -- game loop --

        // Calculate delta time
        let now = Instant::now();
        let delta_time = now - last_time;
        last_time = now;
        // Convert delta time to seconds as a float
        let delta_seconds = delta_time.as_secs_f32();
        // Use delta_seconds for movement, animation, etc.

        let fps = {
            // FPS calculations
            let fps = if delta_seconds > 0.0 {
                1.0 / delta_seconds
            } else {
                0.0
            };
            // Update the moving average
            fps_sum -= fps_values[fps_index];
            fps_values[fps_index] = fps;
            fps_sum += fps;
            fps_index = (fps_index + 1) % FPS_SMOOTHING_FRAMES;

            fps_sum / FPS_SMOOTHING_FRAMES as f32
        };

        // call the gamestate update function
        let update_result = gamestate.update(&delta_time);
        if let Err(update_error) = update_result {
            error!("Error updating gamestate: {:?}", update_error);
            break 'game_loop;
        }
        let gamestate_action = update_result.unwrap();
        match gamestate_action {
            gamestate::GameStateAction::NoOp => {},
            gamestate::GameStateAction::ChangeState(new_state) => {
                gamestate = new_state;
            },
            gamestate::GameStateAction::Exit => {
                break 'game_loop;
            }
        }

        // call the gamestate's draw function
        if let Err(draw_error) = gamestate.draw(fps) {
            error!("Error drawing gamestate: {:?}", draw_error);
            break 'game_loop;
        }

        // call the next frame
        next_frame().await;
    } // -- game loop --
}
