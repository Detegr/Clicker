use animatedrotation::AnimatedRotation;
use pointanimation::PointAnimation;

#[derive(Default)]
pub struct State {
    pub clicks: usize,
    pub candy_rotation: AnimatedRotation,
    pub width: f64,
    pub height: f64,
    pub point_animations: Vec<PointAnimation>,
}

pub static mut STATE: Option<State> = None;

pub fn with_state<F, T>(mut f: F)
where
    F: FnMut(&mut State) -> T,
{
    f(unsafe { STATE.as_mut().unwrap() });
}

wasm_export_unsafe!(init() {
    STATE = Some(Default::default());
});
