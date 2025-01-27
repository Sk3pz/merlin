use std::time::Duration;

use macroquad::{math::{vec2, Vec2}, window::{screen_height, screen_width}};

use crate::aircraft::{Aircraft, AircraftType};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PlayerState {
    Normal,
    TurningRight,
    TurningLeft,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub rotation: f32,

    pub turn_rate: f32,
    pub speed: f32,
    pub throttle_percent: f32,

    pub health: u32,
    pub airbrake: bool,

    pub aircraft: Aircraft,

    pub state: PlayerState,
}

impl Player {
    pub async fn new() -> Result<Self, String> {
        let aircraft = AircraftType::GripenE.get_aircraft().await?;
        Ok(Self {
            pos: vec2(0.0, 0.0),
            rotation: 0.0,

            turn_rate: 0.0004,
            speed: 120.0,
            throttle_percent: 60.0,

            health: aircraft.max_health,
            airbrake: false,

            aircraft: aircraft,

            state: PlayerState::Normal,
        })
    }

    pub fn set_throttle(&mut self, throttle_percent: f32) {
        self.throttle_percent = throttle_percent;
    }

    pub fn get_drag_coefficient(&self) -> f32 {
        let mut drag_coefficient = self.aircraft.drag_base;

        // airbrake
        if self.airbrake {
            drag_coefficient += self.aircraft.airbrake_drag;
        }

        // turning
        if self.state == PlayerState::TurningLeft || self.state == PlayerState::TurningRight {
            drag_coefficient *= self.aircraft.turn_drag;
        }

        return drag_coefficient;
    }

    pub fn get_acceleration(&self) -> f32 {
        let mut acc = Aircraft::calculate_acceleration(self.aircraft.thrust_curve.clone(),
         self.throttle_percent as u32, self.aircraft.thrust_multiplier, self.speed, self.get_drag_coefficient(),
          self.aircraft.reference_area, self.aircraft.mass);

        // if decelerating, remove thrust multiplier from the equation
        if acc < 0.0 {
            acc /= self.aircraft.thrust_multiplier;
        }

        acc
    }

    pub fn get_thrust(&self) -> f32 {
        self.aircraft.thrust_curve.get_thrust(self.throttle_percent as u32, self.aircraft.thrust_multiplier) as f32
    }

    fn update_speed(&mut self, delta_time: f32) {
        let acc = self.get_acceleration();
        self.speed += acc;
        let stall_speed = self.aircraft.stall_speed;
        if self.speed < stall_speed {
            self.speed = stall_speed;
        }
    }

    fn update_turn_rate(&mut self) {
        self.turn_rate = self.aircraft.calc_turn_rate(self.speed);
    }

    fn apply_velocity(&mut self, delta_time: f32) {
        let direction = vec2(self.rotation.sin(), self.rotation.cos());
        let velocity = (direction * self.speed) * vec2(0.002, -0.002);
        self.pos += velocity * delta_time;

        let edge_bounds = 30.0;

        // teleport when at edges
        if self.pos.x > screen_width() + edge_bounds {
            self.pos.x = -edge_bounds;
        } else if self.pos.x < -edge_bounds {
            self.pos.x = screen_width() + edge_bounds;
        }
        
        if self.pos.y > screen_height() + edge_bounds {
            self.pos.y = -edge_bounds;
        } else if self.pos.y < -edge_bounds {
            self.pos.y = screen_height() + edge_bounds;
        }
    }

    pub fn apply_action(&mut self, new_state: PlayerState) {
        self.state = new_state;
    }

    pub fn update(&mut self, delta_time: &Duration) {
        let delta_time = delta_time.as_millis() as f32;

        // update speed
        self.update_speed(delta_time);

        // update turn rate
        self.update_turn_rate();

        // update velocity
        self.apply_velocity(delta_time);

        // handle turning
        match self.state {
            PlayerState::TurningLeft => {
                self.rotation -= self.turn_rate;
            }
            PlayerState::TurningRight => {
                self.rotation += self.turn_rate;
            }
            _ => {}
        }
    }
}