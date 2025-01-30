use std::time::Duration;

use macroquad::{color::{Color, BLACK, WHITE}, text::draw_text, texture::{draw_texture_ex, DrawTextureParams}, window::clear_background};

use crate::{controls::{Action, ControlHandler}, player::{self, PlayerState}};

use super::{GameState, GameStateAction, GameStateError};

pub const THROTTLE_INCREMENTATION: f32 = 1.0;

// TODO: make this a setting
pub const AIRBRAKE_TOGGLE: bool = true;

#[derive(Clone)]
pub struct PlayingGS {
    player: player::Player,
    control_handler: ControlHandler,
}

impl PlayingGS {
    pub async fn new() -> Result<Box<Self>, GameStateError> {
        let player = player::Player::new().await;
        if let Err(e) = player {
            return Err(GameStateError::InitializationError(format!("Failed to initialize player: {}", e)));
        }
        let player = player.unwrap();

        let control_handler = ControlHandler::load();

        Ok(Box::new(Self {
            player,
            control_handler,
        }))
    }

    pub fn reload_controls(&mut self) {
        self.control_handler = ControlHandler::load();
    }
}

impl GameState for PlayingGS {

    fn update(&mut self, delta_time: &Duration) -> Result<GameStateAction, GameStateError> {

        // handle input and make the player respond accordingly
        let actions = self.control_handler.get_actions_down();
        // handle various movement types
        for action in actions {
            match action {
                Action::ThrottleUp => {
                    self.player.set_throttle(self.player.throttle_percent + THROTTLE_INCREMENTATION);
                    if self.player.throttle_percent > 100.0 {
                        self.player.set_throttle(110.0);
                    }
                }
                Action::ThrottleDown => {
                    self.player.set_throttle(self.player.throttle_percent - THROTTLE_INCREMENTATION);
                    if self.player.throttle_percent < 0.0 {
                        self.player.set_throttle(0.0);
                    }
                }
                Action::RollLeft => {
                    if self.player.state == PlayerState::TurningRight {
                        self.player.state = PlayerState::Normal;
                    } else {
                        self.player.apply_action(PlayerState::TurningLeft);
                    }
                }
                Action::RollRight => {
                    if self.player.state == PlayerState::TurningLeft {
                        self.player.state = PlayerState::Normal;
                    } else {
                        self.player.apply_action(PlayerState::TurningRight);
                    }
                }
                Action::Airbrake => {
                    if !AIRBRAKE_TOGGLE {
                        self.player.airbrake = true;
                    }
                }
                _ => {}
            }
        }

        // update the player
        self.player.update(delta_time);

        // handle the pause key with a key release to prevent spamming
        let actions = self.control_handler.get_actions_up();
        for action in actions {
            match action {
                Action::Pause => {
                    return Ok(GameStateAction::ChangeState(Box::new(super::pause::PauseGS::new(self.clone()))))
                }
                Action::ThrottleUp => {
                    if self.player.throttle_percent > 100.0 {
                        self.player.throttle_percent = 100.0;
                    }
                }
                Action::RollLeft => {
                    if self.player.state != PlayerState::TurningRight {
                        self.player.apply_action(PlayerState::Normal);
                    }
                }
                Action::RollRight => {
                    if self.player.state != PlayerState::TurningLeft {
                        self.player.apply_action(PlayerState::Normal);
                    }
                }
                Action::Airbrake => {
                    if AIRBRAKE_TOGGLE {
                        self.player.airbrake = !self.player.airbrake;
                    } else {
                        self.player.airbrake = false;
                    }
                }
                _ => {}
            }
        }

        Ok(GameStateAction::NoOp)
    }

    fn draw(&self, fps: f32) -> Result<(), GameStateError> {
        // clear the background and give a default color
        clear_background(Color::from_rgba(11, 156, 209, 255));

        // draw the player
        draw_texture_ex(
            &self.player.aircraft.sprite, 
            self.player.pos.x, self.player.pos.y,  
            WHITE,
            DrawTextureParams {
                rotation: self.player.rotation,
                ..Default::default()
            }
        );

        // draw the FPS counter in the top right
        draw_text(&format!("FPS:      {}", fps.round()),                        2.0, 12.0 * 1.0, 20.0, BLACK);
        draw_text(&format!("THROTTLE: {}", self.player.throttle_percent),       2.0, 12.0 * 2.0, 20.0, BLACK);
        draw_text(&format!("HEALTH:   {}", self.player.health),                 2.0, 12.0 * 3.0, 20.0, BLACK);
        draw_text(&format!("AIRBRAKE: {}", self.player.airbrake),               2.0, 12.0 * 4.0, 20.0, BLACK);
        draw_text(&format!("SPEED:    {}", self.player.speed),                  2.0, 12.0 * 5.0, 20.0, BLACK);
        draw_text(&format!("ACCL:     {}", self.player.get_acceleration()),     2.0, 12.0 * 6.0, 20.0, BLACK);
        draw_text(&format!("T-RATE:   {}", self.player.turn_rate),              2.0, 12.0 * 7.0, 20.0, BLACK);
        draw_text(&format!("DRAG:     {}", self.player.get_drag_coefficient()), 2.0, 12.0 * 8.0, 20.0, BLACK);
        draw_text(&format!("THRUST:   {}", self.player.get_thrust()),           2.0, 12.0 * 9.0, 20.0, BLACK);

        Ok(())
    }

}