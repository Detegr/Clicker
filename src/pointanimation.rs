use update::Update;

#[derive(Default)]
pub struct PointAnimation {
    pub x: f64,
    pub y: f64,
    pub opacity: f64,
}
impl Update for PointAnimation {
    fn update(&mut self, dt: f64) {
        self.y -= 2.0 * dt;
        self.opacity -= 0.02 * dt;
    }
}
