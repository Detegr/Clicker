use std::default::Default;
use std::os::raw::c_double as jsnum;

#[macro_use]
mod macros;
mod update;
use update::Update;

mod animatedrotation;
use animatedrotation::AnimatedRotation;

mod pointanimation;
use pointanimation::PointAnimation;

#[derive(Default)]
struct State {
    clicks: usize,
    candy_rotation: AnimatedRotation,
    width: f64,
    height: f64,
    point_animations: Vec<PointAnimation>,
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
    fn draw_candy(x: f64, y: f64, rotation: f64);
    fn draw_clicks(clicks: usize);
    fn draw_plus_one(x: f64, y: f64, opacity: f64);
}

wasm_export_unsafe!(init() {
    STATE = Some(Default::default());
});

wasm_export!(resize(w: jsnum, h: jsnum) {
    with_state(|state| {
        state.width = w;
        state.height = h;
    });
});

wasm_export_unsafe!(draw() {
    clear();
    with_state(|state| {
        draw_candy(state.width/2.0, state.height/2.0, state.candy_rotation.get_rotation());
        draw_clicks(state.clicks);
        for anim in &state.point_animations {
            draw_plus_one(anim.x, anim.y, anim.opacity);
        }
    });
});

wasm_export!(click() {
    with_state(|s| {
        s.clicks += 1;
        s.point_animations.push(PointAnimation {
            x: s.width / 2.0,
            y: s.height / 2.0,
            opacity: 1.0,
        });
    });
});

wasm_export!(update(ts: jsnum) {
    let ts = 60.0 / ts;
    with_state(|s| {
        s.candy_rotation.update(ts);

        let mut anims_to_remove = vec![];
        for (i, anim) in s.point_animations.iter_mut().enumerate() {
            anim.update(ts);
            if anim.opacity < 0.0 {
                anims_to_remove.push(i);
            }
        }
        for i in anims_to_remove {
            s.point_animations.remove(i);
        }
    });
});
