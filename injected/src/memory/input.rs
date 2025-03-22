//! Holds all the hooks for the input dll

use retour::static_detour;

use crate::{
    init_enable_hook,
    memory::{AddressLocation, DxVersion},
};

static_detour! {
    pub static GetMouseDeltaX: unsafe extern "system" fn(usize) -> f32;
}

pub fn init(dx_version: DxVersion) {
    let module_name = match dx_version {
        DxVersion::Dx11 => "input_rmdwin7_f.dll",
        DxVersion::Dx12 => "input_rmdwin10_f.dll",
    }
    .into();

    let address = AddressLocation::ModuleExport {
        module_name,
        symbol: "?getMouseDeltaX@InputX86@input@@QEAAMXZ".into(),
    };

    init_enable_hook!(
        GetMouseDeltaX, @ address -> |this| GetMouseDeltaX.call(this);
    );
}
