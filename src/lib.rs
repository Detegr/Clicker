use std::default::Default;
use std::os::raw::c_double as jsnum;

#[derive(Default)]
struct State {
    clicks: usize,
    width: f64,
    height: f64,
    point_animations: Vec<PointAnimation>,
}

#[derive(Default)]
struct PointAnimation {
    x: f64,
    y: f64,
    opacity: f64,
}

static mut STATE: Option<State> = None;

fn with_state<F, T>(mut f: F)
where
    F: FnMut(&mut State) -> T,
{
    f(unsafe { STATE.as_mut().unwrap() });
}

extern "C" {
    fn clear();
    fn draw_clicks(clicks: usize);
    fn draw_plus_one(x: f64, y: f64, opacity: f64);
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    STATE = Some(Default::default());
}

#[no_mangle]
pub extern "C" fn resize(w: jsnum, h: jsnum) {
    with_state(|state| {
        state.width = w;
        state.height = h;
    });
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    clear();
    with_state(|state| {
        draw_clicks(state.clicks);
        for anim in &state.point_animations {
            draw_plus_one(anim.x, anim.y, anim.opacity);
        }
    });
}

#[no_mangle]
pub extern "C" fn click() {
    with_state(|s| {
        s.clicks += 1;
        s.point_animations.push(PointAnimation {
            x: s.width / 2.0,
            y: s.height / 2.0,
            opacity: 1.0,
        });
    });
}

#[no_mangle]
pub extern "C" fn update(ts: jsnum) {
    let ts = 60.0 / ts;
    with_state(|s| {
        let mut anims_to_remove = vec![];
        for (i, anim) in s.point_animations.iter_mut().enumerate() {
            anim.y -= 1.0 * ts;
            anim.opacity -= 0.01 * ts;
            if anim.opacity < 0.0 {
                anims_to_remove.push(i);
            }
        }
        for i in anims_to_remove {
            s.point_animations.remove(i);
        }
    });
}
