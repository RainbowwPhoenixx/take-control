//! Holds all the hooks for the renderer dll

use tracing::{debug, error};

use crate::{
    init_enable_hook,
    memory::{AddressLocation, DxVersion},
};

pub mod shape_engine {
    use crate::common_game_types::Vec2;
    use retour::static_detour;

    static_detour! {
        pub static getInstance: unsafe extern "system" fn() -> usize;
        pub static drawLine: unsafe extern "system" fn(usize, *const Vec2, *const Vec2);
    }
}

pub fn init(dx_version: DxVersion) {
    let module_name: String = match dx_version {
        DxVersion::Dx11 => "renderer_rmdwin7_f.dll",
        DxVersion::Dx12 => "renderer_rmdwin10_f.dll",
    }
    .into();

    let shape_engine_get_instance_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?getInstance@ShapeEngine@rend@@SAPEAV12@XZ".into(),
    };

    let drawline_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?drawLine@ShapeEngine@rend@@QEAAXAEBV?$Vector2Template@M@m@@0@Z".into(),
    };

    init_enable_hook!(
        shape_engine::getInstance, @ shape_engine_get_instance_address -> ||                 shape_engine::getInstance.call();
        shape_engine::drawLine   , @ drawline_address                  -> |this, start, end| shape_engine::drawLine.call(this, start, end);
    );
}
