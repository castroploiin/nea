use std::f32::consts;

struct Angle {
    radians: f32
}

impl Angle {
    fn rad_to_deg(self) -> f32 {
        self.radians * consts::PI/180_f32
    } 
}

mod LinearMotion {
    fn computePosition(position: [&i32; 2], velocity: [&i32; 2], time: f32) {
        position += velocity * time
    }

    fn computeVelocity(velocity: [&i32; 2], acceleration: [&i32; 2], time: f32) -P> [&i32; 2] {
        velocity += acceleration * time
    }
}

fn main() {
    println!("{}", consts::PI)
}