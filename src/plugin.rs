static mut HOOK: Option<Box<rtdhook_rs::callhook::CallHook>> = None;

struct CVector {
    x: f32,
    y: f32,
    _z: f32
}

const GAME_RANGE: (f32, f32) = (180.0f32, 350.0f32);    // MIN, MAX
const CUSTOM_SHIFT: (f32, f32) = (140.0f32, 215.0f32);  // MIN, MAX
const POSITION_SHIFT_VALUE: f32 = ((GAME_RANGE.1 / GAME_RANGE.0) + (CUSTOM_SHIFT.1 / CUSTOM_SHIFT.0)) / 2.0f32;

#[allow(non_snake_case)]
unsafe extern "cdecl" fn CRadar__DrawRadarMap() {
    let radar_orientation = *(0xBA8310 as *const f32);
    let radar_range = *(0xBA8314 as *const f32);
    let position_shift = radar_range / POSITION_SHIFT_VALUE;
    let centre_of_world = find_player_centre_of_world_no_interior_shift(-1);

    *(0xBAA248 as *mut f32) = centre_of_world.x - (radar_orientation - 1.570796370506287f32).cos() * position_shift;
    *(0xBAA24C as *mut f32) = centre_of_world.y - (radar_orientation - 1.570796370506287f32).sin() * position_shift;

    (std::mem::transmute::<usize, extern "cdecl" fn()>(HOOK.as_ref().unwrap().function_ptr()))();
}

fn find_player_centre_of_world_no_interior_shift(player_id: i32) -> CVector {
    let mut ret = CVector {x: 0f32, y: 0f32, _z: 0f32};
    unsafe {
        std::mem::transmute::<usize, extern "cdecl" fn(&mut CVector, i32)>(0x56E400)(&mut ret, player_id);
    };
    ret
}

pub fn init() {
    unsafe {
        match HOOK.as_ref() {
            None => {
                HOOK = Some(Box::new(rtdhook_rs::callhook::CallHook::new(0x586D4E, CRadar__DrawRadarMap as usize)));
                HOOK.as_mut().unwrap().install();
            },
            _ => {}
        }
    }
}
