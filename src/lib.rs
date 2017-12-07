use std::default::Default;
use std::os::raw::c_double as jsnum;

#[macro_use]
mod macros;

enum AnimatedRotation {
    Left(f64),
    Right(f64),
}
impl Default for AnimatedRotation {
    fn default() -> AnimatedRotation {
        AnimatedRotation::Right(0.0)
    }
}
impl AnimatedRotation {
    fn get_rotation(&self) -> f64 {
        match *self {
            AnimatedRotation::Left(val) => val * 3.1415 / 180.0,
            AnimatedRotation::Right(val) => val * 3.1415 / 180.0,
        }
    }
}

#[derive(Default)]
struct State {
    clicks: usize,
    candy_rotation: AnimatedRotation,
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
        let max_rotation = 8.0;
        let rotation_speed = 0.05;
        s.candy_rotation = match s.candy_rotation {
            AnimatedRotation::Left(val) if val < -max_rotation => AnimatedRotation::Right(-max_rotation),
            AnimatedRotation::Left(ref val) => AnimatedRotation::Left(val - rotation_speed * ts),
            AnimatedRotation::Right(val) if val > max_rotation => AnimatedRotation::Left(max_rotation),
            AnimatedRotation::Right(val) => AnimatedRotation::Right(val + rotation_speed * ts),
        };
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
});
