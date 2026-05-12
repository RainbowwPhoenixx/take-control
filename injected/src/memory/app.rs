use std::cell::LazyCell;

use retour::static_detour;
use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{init_enable_hook, memory::DX_VERSION, tas_player::TasPlayer, transparent_call};

use super::{AddressLocation, DxVersion};

const MODULE_NAME: LazyCell<String> = LazyCell::new(|| {
    match *DX_VERSION {
        DxVersion::Dx11 => "app_rmdwin7_f.dll",
        DxVersion::Dx12 => "app_rmdwin10_f.dll",
    }
    .into()
});
pub const G_WINDOW_PTR: LazyCell<AddressLocation> = LazyCell::new(|| AddressLocation::Address {
    module_name: MODULE_NAME.clone(),
    offset: 0x1fc50,
});

pub fn init() {
    let keydown_addr = AddressLocation::PatternScan {
        module_name: MODULE_NAME.clone(),
        pattern: "48 89 5c 24 18 55 56 57 41 56 41 57 48 8b ec 48 83 ec 60 45 33 ff 4d 8b f1 49 8b f8 0f b6 f2 48 8b d9 84 d2".into(),
    };

    let keyup_addr = AddressLocation::PatternScan {
        module_name: MODULE_NAME.clone(),
        pattern: "48 89 5c 24 08 48 89 7c 24 10 55 48 8b ec 48 83 ec 50 48 8b 1d ?? ?? 01 00 48 8b fa 0f b6 c2 b9 a0 00 00 00 c7 45 e0 07 00 00 00 89 7d e4 c7 45 ec 00 00 00 00 c6 44 18 3c 00".into(),
    };

    init_enable_hook!(
        handle_keyup,   @ keyup_addr   -> transparent_call!(handle_keyup, a, b);
        handle_keydown, @ keydown_addr -> custom_handle_keydown;
    );
}

static_detour! {
    pub static handle_keydown: unsafe extern "system" fn(usize, bool, WPARAM, LPARAM);
    // params: some unknown unused ptr, virtual keycode
    pub static handle_keyup: unsafe extern "system" fn(usize, u64);
}

fn custom_handle_keydown(this: usize, down: bool, w: WPARAM, l: LPARAM) {
    if !TasPlayer::get().should_block_user_inputs() {
        unsafe { handle_keydown.call(this, down, w, l) }
    }
}
