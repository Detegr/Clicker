//! JavaScript functions that can be called from this WebAssembly module
extern "C" {
    pub fn clear();
    pub fn draw_candy(x: f64, y: f64, rotation: f64);
    pub fn draw_clicks(clicks: usize);
    pub fn draw_plus_one(x: f64, y: f64, opacity: f64);
}
