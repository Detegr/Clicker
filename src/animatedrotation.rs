use update::Update;

pub enum AnimatedRotation {
    Left(f64),
    Right(f64),
}
impl Default for AnimatedRotation {
    fn default() -> AnimatedRotation {
        AnimatedRotation::Right(0.0)
    }
}
impl AnimatedRotation {
    pub fn get_rotation(&self) -> f64 {
        match *self {
            AnimatedRotation::Left(val) => val * 3.1415 / 180.0,
            AnimatedRotation::Right(val) => val * 3.1415 / 180.0,
        }
    }
}
impl Update for AnimatedRotation {
    fn update(&mut self, dt: f64) {
        const MAX_ROTATION: f64 = 8.0;
        const ROTATION_SPEED: f64 = 0.1;

        *self = match *self {
            AnimatedRotation::Left(val) if val < -MAX_ROTATION => AnimatedRotation::Right(
                -MAX_ROTATION,
            ),
            AnimatedRotation::Left(ref val) => AnimatedRotation::Left(val - ROTATION_SPEED * dt),
            AnimatedRotation::Right(val) if val > MAX_ROTATION => AnimatedRotation::Left(
                MAX_ROTATION,
            ),
            AnimatedRotation::Right(val) => AnimatedRotation::Right(val + ROTATION_SPEED * dt),
        };
    }
}
