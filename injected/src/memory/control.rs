use retour::static_detour;

use crate::{
    init_enable_hook,
    memory::AddressLocation,
    tas_player::{PlaybackState, TasPlayer},
};

use super::DxVersion;

static_detour! {
    // pub static creategame1: unsafe extern "system" fn(usize); // 1405467e0
    // pub static join_or_host_lan: unsafe extern "system" fn(usize); // 14053eab0
    // pub static start_local: unsafe extern "system" fn(usize, *mut ControlString, *mut ControlString, *mut ControlString, bool); // 14053f290
    pub static update_world: unsafe extern "system" fn(usize);
}

pub fn init() {
    let module_name: String = match *crate::memory::DX_VERSION {
        DxVersion::Dx11 => "Control_DX11.exe",
        DxVersion::Dx12 => "Control_DX12.exe",
    }
    .into();

    let update_world_address_current = AddressLocation::PatternScan {
        module_name: module_name.clone(),
        pattern: "48 8b c4 55 53 56 57 41 54 41 55 41 56 41 57 48 8d 68 a8 48 81 ec 18 01 00 00 48 c7 45 a8 fe ff ff ff 0f 29 70 a8 44 0f 29 40 98 48 8b f1 b1 01".into(),
    };

    let update_world_address_legacy = AddressLocation::PatternScan {
        module_name: module_name.clone(),
        pattern: "48 8b c4 55 53 56 57 41 54 41 55 41 56 41 57 48 8d 6c 24 b8 48 81 ec 48 01 00 00 48 c7 45 d8 fe ff ff ff 0f 29 70 a8 44 0f 29 40 98 48 8b f1 48 8d 8d 90 00 00 00".into()
    };

    let succeed = init_enable_hook!(
        update_world, @ update_world_address_current -> custom_update_world;
    );

    if !succeed {
        tracing::info!("Failed to hook current patch update world, retrying with legacy");
        let legacy_success = init_enable_hook!(
            update_world, @ update_world_address_legacy -> custom_update_world;
        );
    }
}

fn custom_update_world(this: usize) {
    let mut tas_player = TasPlayer::get();
    if tas_player.state == PlaybackState::Paused {
        return;
    }

    if tas_player.state != PlaybackState::Stopped {
        tas_player.update_inputs();
    }
    drop(tas_player);

    unsafe { update_world.call(this) };
}
