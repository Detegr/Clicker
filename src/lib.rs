use std::os::raw::c_double as jsnum;

#[macro_use]
mod macros;
mod update;
use update::Update;

mod animatedrotation;

mod gamestate;
use gamestate::with_state;
pub use gamestate::init;

mod pointanimation;
use pointanimation::PointAnimation;

mod wasm;

wasm_export!(resize(w: jsnum, h: jsnum) {
    with_state(|state| {
        state.width = w;
        state.height = h;
    });
});

wasm_export_unsafe!(draw() {
    wasm::clear();
    with_state(|state| {
        wasm::draw_candy(state.width/2.0, state.height/2.0, state.candy_rotation.get_rotation());
        wasm::draw_clicks(state.clicks);
        for anim in &state.point_animations {
            wasm::draw_plus_one(anim.x, anim.y, anim.opacity);
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
