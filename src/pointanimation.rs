use update::Update;

#[derive(Default)]
pub struct PointAnimation {
    pub x: f64,
    pub y: f64,
    pub opacity: f64,
}
impl Update for PointAnimation {
    fn update(&mut self, ts: f64) {
        self.y -= 1.0 * ts;
        self.opacity -= 0.01 * ts;
    }
}
