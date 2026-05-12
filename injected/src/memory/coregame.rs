use crate::{init_enable_hook, log_call};

use super::{AddressLocation, DxVersion};

#[allow(non_snake_case)]
pub fn init() {
    let module_name: String = match *crate::memory::DX_VERSION {
        DxVersion::Dx11 => "coregame_rmdwin7_f.dll",
        DxVersion::Dx12 => "coregame_rmdwin10_f.dll",
    }
    .into();

    let hostLanMultiplayerGame_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?hostLanMultiplayerGame@GameSessionParameters@coregame@@SA?AV12@PEBD00M@Z".into(),
    };
    let joinLanMultiplayerGame_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?joinLanMultiplayerGame@GameSessionParameters@coregame@@SA?AV12@AEBVNetworkAddress@net@@@Z".into(),
    };
    let joinPlatformSessionGame_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?joinPlatformSessionGame@GameSessionParameters@coregame@@SA?AV12@PEBD@Z".into(),
    };
    let loadSinglePlayerGame_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?loadSinglePlayerGame@GameSessionParameters@coregame@@SA?AV12@PEBD00M@Z".into(),
    };
    let startAutoJoinMultiplayerGame_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?startAutoJoinMultiplayerGame@GameSessionParameters@coregame@@SA?AV12@PEBD00M@Z"
            .into(),
    };
    let startLocalMultiplayerPlayerLevel_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?startLocalMultiplayerPlayerLevel@GameSessionParameters@coregame@@SA?AV12@AEBVString@r@@00_NM@Z".into(),
    };
    let startSinglePlayerLevel_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol:
            "?startSinglePlayerLevel@GameSessionParameters@coregame@@SA?AV12@AEBVString@r@@00_N1M@Z"
                .into(),
    };

    init_enable_hook!(
        game_session_parameters::hostLanMultiplayerGame,           @ hostLanMultiplayerGame_address           -> log_call!(game_session_parameters::hostLanMultiplayerGame, this, s1, s2, s3, f);
        game_session_parameters::joinLanMultiplayerGame,           @ joinLanMultiplayerGame_address           -> log_call!(game_session_parameters::joinLanMultiplayerGame, this, address);
        game_session_parameters::joinPlatformSessionGame,          @ joinPlatformSessionGame_address          -> log_call!(game_session_parameters::joinPlatformSessionGame, this, s);
        game_session_parameters::loadSinglePlayerGame,             @ loadSinglePlayerGame_address             -> log_call!(game_session_parameters::loadSinglePlayerGame, this, s1, s2, s3, f);
        game_session_parameters::startAutoJoinMultiplayerGame,     @ startAutoJoinMultiplayerGame_address     -> log_call!(game_session_parameters::startAutoJoinMultiplayerGame, this, s1, s2, s3, f);
        game_session_parameters::startLocalMultiplayerPlayerLevel, @ startLocalMultiplayerPlayerLevel_address -> log_call!(game_session_parameters::startLocalMultiplayerPlayerLevel, this, s1, s2, s3, b, f);
        game_session_parameters::startSinglePlayerLevel,           @ startSinglePlayerLevel_address           -> log_call!(game_session_parameters::startSinglePlayerLevel, this, s1, s2, s3, b1, b2, f);
    );
}

pub mod game_session_parameters {
    use my_proc_macros::PubDebug;
    use retour::static_detour;

    use crate::{common_game_types::InplaceString, memory::net::network_address::NetworkAddress};

    use super::GameSessionType;

    #[derive(PubDebug)]
    #[repr(C)]
    pub struct GameSessionParameters {
        pub session_type: GameSessionType,
        _pad1: u32,
        pub str1: InplaceString<64>,
        pub str2: InplaceString<64>,
        pub str3: InplaceString<64>,
        pub unk1: bool,
        _pad2: [u8; 11],
        pub network_address: NetworkAddress,
        pub str4: InplaceString<64>,
        pub thread_model: u32, // (enum GameSessionServerThreadModel)
        pub unk2: f32,
    }

    static_detour! {
        pub static hostLanMultiplayerGame:           unsafe extern "system" fn(*mut GameSessionParameters, *const char, *const char, *const char, f32) -> *mut GameSessionParameters;
        pub static joinLanMultiplayerGame:           unsafe extern "system" fn(*mut GameSessionParameters, *const NetworkAddress) -> *mut GameSessionParameters;
        pub static joinPlatformSessionGame:          unsafe extern "system" fn(*mut GameSessionParameters, *const char) -> *mut GameSessionParameters;
        // Called on load game/mission
        pub static loadSinglePlayerGame:             unsafe extern "system" fn(*mut GameSessionParameters, *const char, *const char, *const char, f32) -> *mut GameSessionParameters;
        pub static startAutoJoinMultiplayerGame:     unsafe extern "system" fn(*mut GameSessionParameters, *const char, *const char, *const char, f32) -> *mut GameSessionParameters;
        pub static startLocalMultiplayerPlayerLevel: unsafe extern "system" fn(*mut GameSessionParameters, *const String, *const String, *const String, bool, f32) -> *mut GameSessionParameters;
        // Called on new game
        pub static startSinglePlayerLevel:           unsafe extern "system" fn(*mut GameSessionParameters, *const String, *const String, *const String, bool, bool, f32) -> *mut GameSessionParameters;
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum GameSessionType {
    Singleplayer,
    LocalMultiplayer,
    LanMultiplayer,
}

pub mod character_controller_component_state {
    use crate::memory::rl::component_state_base::ComponentStateBase;

    #[repr(C)]
    pub struct CharacterControllerComponentState {
        base: ComponentStateBase,
        _pad1: [u8; 0x40],
        pub posang: [f32; 8],
    }
}

pub mod character_controller_component {
    use crate::memory::{
        physics::character_controller::CharacterController, rl::component_base::ComponentBase,
    };

    #[repr(C)]
    pub struct CharacterControllerComponent {
        base: ComponentBase,
        _pad1: [u8; 0x8],
        character_controller: *mut CharacterController,
    }
}
