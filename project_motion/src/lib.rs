#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        // Update time by 1 second
        self.time += 1.0;
        
        // Calculate new position using projectile motion equations
        let new_x = truncate_precision(
            self.init_position.x + self.init_velocity.x * self.time
        );
        
        let new_y = truncate_precision(
            self.init_position.y + 
            self.init_velocity.y * self.time -
            0.5 * 9.8 * self.time * self.time
        );
        
        // Calculate new velocity components
        let vel_x = truncate_precision(self.init_velocity.x);
        let vel_y = truncate_precision(self.init_velocity.y - 9.8 * self.time);
        
        // Create updated position and velocity
        let new_position = Object { x: new_x, y: new_y };
        let new_velocity = Object { x: vel_x, y: vel_y };
        
        // Check if object has hit the ground
        if new_position.y < 0.0 {
            return None;
        }
        
        // Return updated object state
        Some(ThrowObject {
            init_position: self.init_position.clone(),
            init_velocity: self.init_velocity.clone(),
            actual_position: new_position,
            actual_velocity: new_velocity,
            time: self.time,
        })
    }
}

// Helper function to round to 2 decimal places
fn truncate_precision(value: f32) -> f32 {
    (value * 100.0).round() / 100.0
}