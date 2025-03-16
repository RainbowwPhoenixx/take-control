use crate::init_enable_hook;

use super::{AddressLocation, DxVersion};

pub mod physics_scene {
    use retour::static_detour;

    static_detour! {
        pub static startStep: unsafe extern "system" fn(usize);
    }
}

pub fn init(dx_version: DxVersion) {
    let module_name: String = match dx_version {
        DxVersion::Dx11 => "physics_rmdwin7_f.dll",
        DxVersion::Dx12 => "physics_rmdwin10_f.dll",
    }
    .into();

    let start_step_addr = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?startStep@PhysicsScene@physics@@QEAAXXZ".into(),
    };

    init_enable_hook!(
        physics_scene::startStep, @ start_step_addr -> |this| physics_scene::startStep.call(this),
    );
}
