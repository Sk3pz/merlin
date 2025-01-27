use std::time::Duration;

use macroquad::{color::Color, shapes::draw_rectangle, window::{screen_height, screen_width}};

use crate::controls::ControlHandler;

use super::{playing::PlayingGS, GameState, GameStateAction, GameStateError};

pub struct PauseGS {
    previous_play_state: PlayingGS,
}

impl PauseGS {
    pub fn new(previous_play_state: PlayingGS) -> Self {
        Self {
            previous_play_state,
        }
    }
}

impl GameState for PauseGS {

    fn update(&mut self, _delta_time: &Duration) -> Result<GameStateAction, GameStateError> {
        let control_handler = ControlHandler::load();
        // handle on release to ensure pause key isnt spammed when held (was an issue)
        let actions = control_handler.get_actions_up();
        for action in actions {
            match action {
                crate::controls::Action::Pause => {
                    self.previous_play_state.reload_controls();
                    return Ok(GameStateAction::ChangeState(Box::new(self.previous_play_state.clone())));
                }
                _ => {}
            }
        }
        Ok(GameStateAction::NoOp)
    }

    fn draw(&self, fps: f32) -> Result<(), GameStateError> {
        // draw the player in the correct position
        // todo: this will not show updates to other players surrounding the player when networking is implemented, the update function may need to be called with a special pause flag?
        self.previous_play_state.draw(fps)?;

        // draw a semi-transparent overlay
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));

        Ok(())
    }
    
}