use std::collections::HashMap;

use macroquad::texture::{load_texture, Texture2D};

pub enum AircraftType {
    F16,
    GripenE,
    X47B,
}

impl AircraftType {

    pub async fn get_aircraft(&self) -> Result<Aircraft, String> {
        Ok(match self {
            Self::F16 => {
                let texture = load_texture("assets/sprites/aircraft/f16_level.png").await;
                if let Err(e) = texture {
                    return Err(e.to_string())
                }
                let texture = texture.unwrap();

                let mut thrust_curve = ThrustCurve::new();
                thrust_curve.add_point(0, 0);      // At 0% throttle, no thrust
	            thrust_curve.add_point(25, 500);   // At 25% throttle, 500 units of thrust
	            thrust_curve.add_point(50, 1500);  // At 50% throttle, 1500 units of thrust
	            thrust_curve.add_point(75, 3000);  // At 75% throttle, 3000 units of thrust
	            thrust_curve.add_point(85, 4000);  // At 85% throttle, 4000 units of thrust
	            thrust_curve.add_point(90, 4800);  // At 90% throttle, 4800 units of thrust
	            thrust_curve.add_point(100, 5000); // At 100% throttle, 5000 units of thrust
	            thrust_curve.add_point(110, 6000); // At 110% throttle, 5500 units of thrust

                Aircraft {
                    name: "F-16".to_string(),
                    sprite: texture,
                    max_health: 100,

                    base_turn_rate: 0.04,
                    turn_flip_point: 100.0,
                    max_turn_rate: 0.06,
                    min_turn_rate: 0.025,
                    
                    stall_speed: 50.0,

                    drag_base: 0.02,
                    turn_drag: 1.2,

                    airbrake_drag: 0.06,

                    reference_area: 30.0,
                    mass: 8000.0,
                    thrust_multiplier: 4.9090909,

                    bullet_fire_rate: 10, // ms

                    thrust_curve,
                }
            }
            Self::GripenE => {
                let texture = load_texture("assets/sprites/aircraft/gripen_level.png").await;
                    if let Err(e) = texture {
                        return Err(e.to_string())
                    }
                    let texture = texture.unwrap();
    
                    let mut thrust_curve = ThrustCurve::new();
                    thrust_curve.add_point(0, 0);      // At 0% throttle, no thrust
                    thrust_curve.add_point(25, 500);   // At 25% throttle, 500 units of thrust
                    thrust_curve.add_point(50, 1500);  // At 50% throttle, 1500 units of thrust
                    thrust_curve.add_point(75, 3000);  // At 75% throttle, 3000 units of thrust
                    thrust_curve.add_point(85, 4000);  // At 85% throttle, 4000 units of thrust
                    thrust_curve.add_point(90, 4800);  // At 90% throttle, 4800 units of thrust
                    thrust_curve.add_point(100, 5000); // At 100% throttle, 5000 units of thrust
                    thrust_curve.add_point(110, 5500); // At 110% throttle, 5500 units of thrust
    
                    Aircraft {
                        name: "Gripen".to_string(),
                        sprite: texture,
                        max_health: 100,
    
                        base_turn_rate: 0.04,
                        turn_flip_point: 100.0,
                        max_turn_rate: 0.06,
                        min_turn_rate: 0.025,
                        
                        stall_speed: 50.0,
    
                        drag_base: 0.02,
                        turn_drag: 1.2,
    
                        airbrake_drag: 0.06,
    
                        reference_area: 30.0,
                        mass: 8000.0,
                        thrust_multiplier: 4.9090909,
    
                        bullet_fire_rate: 10, // ms
    
                        thrust_curve,
                    }
            }
            _ => {
                return Err("Aircraft not implemented".to_string())
            }
        })
    }

}

#[derive(Debug, Clone)]
pub struct Aircraft {
    pub name: String,
    pub sprite: Texture2D,
    pub max_health: u32, // should scale with size of plane and other aspects (i.e. a-10 has massive health but an f-16 has way smaller health)

    pub base_turn_rate: f32,
    pub turn_flip_point: f32,
    pub max_turn_rate: f32,
    pub min_turn_rate: f32,

    pub stall_speed: f32,

    pub drag_base: f32,
    pub turn_drag: f32,
    pub airbrake_drag: f32,

    pub reference_area: f32, // in m^2
    pub mass: f32, // in kg
    pub thrust_multiplier: f32,

    pub bullet_fire_rate: u32, // ms

    pub thrust_curve: ThrustCurve,
}

impl Aircraft {

    pub fn calc_turn_rate(&self, speed: f32) -> f32 {
        // dark magic with linear regression (I dont understand this)
        let midpoint = self.turn_flip_point;
        let base_turn_rate = self.base_turn_rate;

        let turn_rate = if speed <= midpoint {
            base_turn_rate + (midpoint - speed) * (base_turn_rate / 150.0)
        } else {
            base_turn_rate - (speed - midpoint) * (base_turn_rate / 150.0)
        };

        // clamp to the aircraft's abilities
        self.min_turn_rate.max(turn_rate.min(self.max_turn_rate))
    }

    // calculate acceleration based on throttle percentage, speed, drag and etc
    pub fn calculate_acceleration(thrust_curve: ThrustCurve, throttle_percent: u32, thrust_multiplier: f32, speed: f32, drag_coefficient: f32, reference_area: f32, mass: f32) -> f32 {
        // get the thrust output from the throttle percentage
        let thrust = thrust_curve.get_thrust(throttle_percent, thrust_multiplier);
        // calculate the drag force based on speed
        let drag = 0.5 * drag_coefficient * reference_area * speed * speed;
        // calculate the net force (thrust - drag)
        let net_force = thrust - drag;
        // calculate the acceleration and return (F = ma => a = F/m)
        net_force / mass
    }

}

#[derive(Debug, Clone)]
pub struct ThrustCurve {
    pub points: HashMap<u32, u32>,
}

impl ThrustCurve {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
        }
    }

    pub fn add_point(&mut self, throttle: u32, thrust: u32) {
        self.points.insert(throttle, thrust);
    }

    pub fn get_thrust(&self, throttle_percent: u32, thrust_multiplier: f32) -> f32 {
        // If there are no points, return 0 thrust
        if self.points.is_empty() {
            return 0.0;
        }

        // Collect the keys and sort them
        let mut keys: Vec<&u32> = self.points.keys().collect();
        keys.sort();

        // Find the surrounding throttle points
        let mut lower_key = keys[0];
        let mut upper_key = keys[0];

        for &key in &keys {
            if key <= &throttle_percent {
                lower_key = key;
            }
            if key >= &throttle_percent {
                upper_key = key;
                break;
            }
        }

        // If exact match, return the thrust at that throttle
        if lower_key == upper_key {
            return *self.points.get(lower_key).unwrap() as f32 * thrust_multiplier;
        }

        // Perform linear interpolation
        let lower_thrust = *self.points.get(lower_key).unwrap() as f32;
        let upper_thrust = *self.points.get(upper_key).unwrap() as f32;

        let fraction = (throttle_percent - lower_key) as f32 / (upper_key - lower_key) as f32;
        let interpolated_thrust = lower_thrust + fraction * (upper_thrust - lower_thrust);

        interpolated_thrust * thrust_multiplier
    }
}