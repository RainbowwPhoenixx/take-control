use character_controller::CharacterControllerState;

use crate::init_enable_hook;

use super::{AddressLocation, DxVersion};

pub mod physics_scene {
    use retour::static_detour;

    static_detour! {
        pub static startStep: unsafe extern "system" fn(usize);
    }
}

pub mod character_controller {
    use my_proc_macros::PubDebug;
    use physx::controller::PxCapsuleController;
    use retour::static_detour;

    use crate::common_game_types::{SIMDTransform, Vec2, Vec4};

    static_detour! {
        pub static CharacterController_ctor: unsafe extern "system" fn(*mut CharacterController, *mut CharacterControllerState, usize, i32) -> *mut CharacterController;
    }

    #[derive(Debug)]
    #[repr(C)]
    pub struct CharacterControllerState {
        pub pad: [u8; 0x78],
        pub transfrom: SIMDTransform,
        pub height: f32,
        pub radius: f32,
        pub max_step_height: f32,
        pub is_player: bool,
        pub pad2: bool,
        pub is_using_gravity: bool,
    }

    #[derive(PubDebug)]
    #[repr(C)]
    pub struct CharacterController {
        pub character_controller_state: *mut CharacterControllerState,
        _pad1: [u8; 0x2C],
        pub is_supported: bool,
        pub has_ground_gap: bool,
        pub force_slide: bool,
        pub fall_stopped: bool,
        _pad2: [u8; 0x8],
        pub vel: Vec4<f32>,
        pub ground_vel: Vec2<f64>,
        pub px_controller: *mut PxCapsuleController<()>,
    }

    pub static mut CONTROLLER: *mut CharacterController = std::ptr::null_mut();
}

pub fn init() {
    let module_name: String = match *crate::memory::DX_VERSION {
        DxVersion::Dx11 => "physics_rmdwin7_f.dll",
        DxVersion::Dx12 => "physics_rmdwin10_f.dll",
    }
    .into();

    let start_step_addr = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?startStep@PhysicsScene@physics@@QEAAXXZ".into(),
    };
    let char_controller_ctor_addr = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "??0CharacterController@physics@@QEAA@AEAVCharacterControllerState@1@PEAVGameObject@r@@H@Z".into(),
    };

    let char_ctor = |this, state: *mut CharacterControllerState, b, c| unsafe {
        if state.as_ref().unwrap().is_player {
            character_controller::CONTROLLER = this;
            state.as_mut().unwrap().is_using_gravity = false;
        }

        character_controller::CharacterController_ctor.call(this, state, b, c)
    };

    init_enable_hook!(
        physics_scene::startStep,                       @ start_step_addr ->           |this| physics_scene::startStep.call(this);
        character_controller::CharacterController_ctor, @ char_controller_ctor_addr -> char_ctor;
    );
}
