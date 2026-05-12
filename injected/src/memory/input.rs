//! Holds all the hooks for the input dll

use retour::static_detour;

use crate::{
    init_enable_hook,
    memory::{AddressLocation, DxVersion},
};

static_detour! {
    pub static GetMouseDeltaX: unsafe extern "system" fn(usize) -> f32;
}

pub fn init() {
    let module_name: String = match *crate::memory::DX_VERSION {
        DxVersion::Dx11 => "input_rmdwin7_f.dll",
        DxVersion::Dx12 => "input_rmdwin10_f.dll",
    }
    .into();

    let get_mouse_deltax_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?getMouseDeltaX@InputX86@input@@QEAAMXZ".into(),
    };
    let get_instance_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?getInstance@InputManager@input@@SAPEAV12@XZ".into(),
    };
    let get_action_name_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?getActionName@InputManager@input@@QEAA?AV?$InplaceString@$0IA@@r@@H@Z".into(),
    };
    let get_action_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?getAction@InputManager@input@@QEAAHAEBV?$InplaceWString@$0IA@@r@@@Z".into(),
    };
    let get_gamepad_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?getGamepad@InputManager@input@@QEAAPEAVGamepad@2@XZ".into(),
    };
    let set_console_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?setConsole@InputManager@input@@QEAAX_N@Z".into(),
    };
    let get_axis_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?getAxis@Gamepad@input@@QEBAMH@Z".into(),
    };
    let kb_key_down_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?keyDown@Keyboard@input@@QEBA_NH@Z".into(),
    };
    let kb_key_pressed_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?keyPressed@Keyboard@input@@QEBA_NH@Z".into(),
    };
    let i_key_down_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?keyDown@InputX86@input@@QEBA_NG@Z".into(),
    };
    let i_key_pressed_address = AddressLocation::ModuleExport {
        module_name: module_name.clone(),
        symbol: "?keyClicked@InputX86@input@@QEAA_NG@Z".into(),
    };

    init_enable_hook!(
        GetMouseDeltaX,               @ get_mouse_deltax_address -> |this| GetMouseDeltaX.call(this);
        input_manager::getInstance,   @ get_instance_address     -> || input_manager::getInstance.call();
        input_manager::getAction    , @ get_action_address       -> |this, string| input_manager::getAction.call(this, string);
        input_manager::getActionName, @ get_action_name_address  -> |this, string, action_id| input_manager::getActionName.call(this, string, action_id);
        input_manager::getGamepad,    @ get_gamepad_address      -> |this| input_manager::getGamepad.call(this);
        input_manager::setConsole,    @ set_console_address      -> |this, value| input_manager::setConsole.call(this, value);
        gamepad::getAxis,             @ get_axis_address         -> |this, axis| gamepad::getAxis.call(this, axis);
        keyboard::keyDown,            @ kb_key_down_address         -> |this, key| {let res = keyboard::keyDown.call(this, key); debug!("keyboard::keyDown({key}) -> {res}"); res};
        keyboard::keyPressed,         @ kb_key_pressed_address      -> |this, key| {let res = keyboard::keyPressed.call(this, key); debug!("keyboard::keyPressed({key}) -> {res}"); res};
        input_x86::keyDown,           @ i_key_down_address         -> |this, key| {let res = input_x86::keyDown.call(this, key); debug!("input_x86::keyDown({key}) -> {res}"); res};
        input_x86::keyClicked,        @ i_key_pressed_address      -> |this, key| {let res = input_x86::keyClicked.call(this, key); debug!("input_x86::keyClicked({key}) -> {res}"); res};
    );
}

pub mod input_manager {
    use my_proc_macros::PubDebug;
    use retour::static_detour;

    use crate::common_game_types::InplaceString;

    use super::gamepad;

    #[repr(C)]
    #[derive(PubDebug)]
    pub struct InputManager {
        _pad: [u8; 0x74],
        pub in_menu: bool,
        _pad2: u8,
        pub in_conversation: bool,
        pub in_video: bool,
        pub is_console_on: bool,
        pub is_loading: bool,
        pub binding: bool,
        pub freecamera_without_player_control: bool,
        pub redirect_to_keyframer: bool,
        _pad3: u8,
        _pad4: u8,
        pub invert_vertical_pad: bool,
        pub invert_horizontal_pad: bool,
        pub invert_vertical_mouse: bool,
        pub inver_horizontal_mouse: bool,
        pub sticks_swapped: bool,
        pub script_component_input: bool,
    }

    static_detour! {
        pub static getInstance: unsafe extern "system" fn() -> *mut InputManager;
        pub static getAction: unsafe extern "system" fn(*mut InputManager, *mut InplaceString<128>) -> u32;
        pub static getActionName: unsafe extern "system" fn(*mut InputManager, *mut InplaceString<128>, u32) -> *mut InplaceString<128>;
        pub static getGamepad: unsafe extern "system" fn(*mut InputManager) -> *mut gamepad::Gamepad;
        pub static setConsole: unsafe extern "system" fn(*mut InputManager, bool);
    }
}

pub mod gamepad {
    use retour::static_detour;

    pub struct Gamepad {}
    #[repr(C)]
    // TODO: confirm that this is the actual ordrer
    pub enum GamepadAxis {
        LeftX = 0,
        LeftY = 1,
        RightX = 2,
        RightY = 3,
    }

    static_detour! {
        pub static getAxis: unsafe extern "system" fn(*mut Gamepad, GamepadAxis) -> f32;
    }
}

pub mod keyboard {
    use retour::static_detour;

    static_detour! {
        pub static keyDown: unsafe extern "system" fn(usize, u32) -> bool;
        pub static keyPressed: unsafe extern "system" fn(usize, u32) -> bool;
    }
}

pub mod input_x86 {
    use retour::static_detour;

    static_detour! {
        pub static keyDown: unsafe extern "system" fn(usize, u16) -> bool;
        pub static keyClicked: unsafe extern "system" fn(usize, u16) -> bool;
    }
}

enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MovementX,
    MovementY,
    CameraX,
    CameraY,
    AbortVideo,
    AbortVideoConfirm,
    AbortTimeline,
    AbortTimelineConfirm,
    ScrollTimelinesUp,
    ScrollTimelinesDown,
    LoadingContinue,
    ToggleMenu,
    ToggleGameplayMenu,
    ToggleGameplayMenuDown,
    MenuA,
    MenuADown,
    MenuB,
    MenuBDown,
    MenuX,
    MenuXDown,
    MenuY,
    MenuYDown,
    MenuStart,
    MenuBack,
    MenuLeftthumb,
    MenuRightthumb,
    MenuLeftshoulder,
    MenuRightshoulder,
    MenuPadup,
    MenuPaddown,
    MenuPadleft,
    MenuPadright,
    MenuLeftthumbx,
    MenuLeftthumby,
    MenuRightthumbx,
    MenuRightthumby,
    MenuLefttrigger,
    MenuRighttrigger,
    PresentationLeft,
    PresentationRight,
    FreecameraForward1,
    FreecameraForward2,
    FreecameraBackward1,
    FreecameraBackward2,
    FreecameraX,
    FreecameraZ,
    FreecameraHeading,
    FreecameraPitch,
    FreecameraUp,
    FreecameraDown,
    FreecameraFaster,
    FreecameraSlower,
    FreecameraScaleTranslationUp,
    FreecameraScaleTranslationDown,
    FreecameraScaleRotationUp,
    FreecameraScaleRotationDown,
    FreecameraZoomIn,
    FreecameraZoomOut,
    FreecameraLensPrev,
    FreecameraLensNext,
    PhotomodeX,
    PhotomodeZ,
    PhotomodeUp,
    PhotomodeDown,
    PhotomodeHeading,
    PhotomodePitch,
    PhotomodeEnableCameraMovement,
    PhotomodeHideMenu,
    PhotomodeReset,
    PhotomodeTakePhotoPc,
    Pause,
    DebugEnable,
    DebugDisable,
    DebugSelectHidden,
}

impl Into<String> for Action {
    fn into(self) -> String {
        match self {
            Action::MoveUp => "MOVE_UP",
            Action::MoveDown => "MOVE_DOWN",
            Action::MoveLeft => "MOVE_LEFT",
            Action::MoveRight => "MOVE_RIGHT",
            Action::MovementX => "MOVEMENT_X",
            Action::MovementY => "MOVEMENT_Y",
            Action::CameraX => "CAMERA_X",
            Action::CameraY => "CAMERA_Y",
            Action::AbortVideo => "ABORT_VIDEO",
            Action::AbortVideoConfirm => "ABORT_VIDEO_CONFIRM",
            Action::AbortTimeline => "ABORT_TIMELINE",
            Action::AbortTimelineConfirm => "ABORT_TIMELINE_CONFIRM",
            Action::ScrollTimelinesUp => "SCROLL_TIMELINES_UP",
            Action::ScrollTimelinesDown => "SCROLL_TIMELINES_DOWN",
            Action::LoadingContinue => "LOADING_CONTINUE",
            Action::ToggleMenu => "TOGGLE_MENU",
            Action::ToggleGameplayMenu => "TOGGLE_GAMEPLAY_MENU",
            Action::ToggleGameplayMenuDown => "TOGGLE_GAMEPLAY_MENU_DOWN",
            Action::MenuA => "MENU_A",
            Action::MenuADown => "MENU_A_DOWN",
            Action::MenuB => "MENU_B",
            Action::MenuBDown => "MENU_B_DOWN",
            Action::MenuX => "MENU_X",
            Action::MenuXDown => "MENU_X_DOWN",
            Action::MenuY => "MENU_Y",
            Action::MenuYDown => "MENU_Y_DOWN",
            Action::MenuStart => "MENU_START",
            Action::MenuBack => "MENU_BACK",
            Action::MenuLeftthumb => "MENU_LEFTTHUMB",
            Action::MenuRightthumb => "MENU_RIGHTTHUMB",
            Action::MenuLeftshoulder => "MENU_LEFTSHOULDER",
            Action::MenuRightshoulder => "MENU_RIGHTSHOULDER",
            Action::MenuPadup => "MENU_PADUP",
            Action::MenuPaddown => "MENU_PADDOWN",
            Action::MenuPadleft => "MENU_PADLEFT",
            Action::MenuPadright => "MENU_PADRIGHT",
            Action::MenuLeftthumbx => "MENU_LEFTTHUMBX",
            Action::MenuLeftthumby => "MENU_LEFTTHUMBY",
            Action::MenuRightthumbx => "MENU_RIGHTTHUMBX",
            Action::MenuRightthumby => "MENU_RIGHTTHUMBY",
            Action::MenuLefttrigger => "MENU_LEFTTRIGGER",
            Action::MenuRighttrigger => "MENU_RIGHTTRIGGER",
            Action::PresentationLeft => "PRESENTATION_LEFT",
            Action::PresentationRight => "PRESENTATION_RIGHT",
            Action::FreecameraForward1 => "FREECAMERA_FORWARD1",
            Action::FreecameraForward2 => "FREECAMERA_FORWARD2",
            Action::FreecameraBackward1 => "FREECAMERA_BACKWARD1",
            Action::FreecameraBackward2 => "FREECAMERA_BACKWARD2",
            Action::FreecameraX => "FREECAMERA_X",
            Action::FreecameraZ => "FREECAMERA_Z",
            Action::FreecameraHeading => "FREECAMERA_HEADING",
            Action::FreecameraPitch => "FREECAMERA_PITCH",
            Action::FreecameraUp => "FREECAMERA_UP",
            Action::FreecameraDown => "FREECAMERA_DOWN",
            Action::FreecameraFaster => "FREECAMERA_FASTER",
            Action::FreecameraSlower => "FREECAMERA_SLOWER",
            Action::FreecameraScaleTranslationUp => "FREECAMERA_SCALE_TRANSLATION_UP",
            Action::FreecameraScaleTranslationDown => "FREECAMERA_SCALE_TRANSLATION_DOWN",
            Action::FreecameraScaleRotationUp => "FREECAMERA_SCALE_ROTATION_UP",
            Action::FreecameraScaleRotationDown => "FREECAMERA_SCALE_ROTATION_DOWN",
            Action::FreecameraZoomIn => "FREECAMERA_ZOOM_IN",
            Action::FreecameraZoomOut => "FREECAMERA_ZOOM_OUT",
            Action::FreecameraLensPrev => "FREECAMERA_LENS_PREV",
            Action::FreecameraLensNext => "FREECAMERA_LENS_NEXT",
            Action::PhotomodeX => "PHOTOMODE_X",
            Action::PhotomodeZ => "PHOTOMODE_Z",
            Action::PhotomodeUp => "PHOTOMODE_UP",
            Action::PhotomodeDown => "PHOTOMODE_DOWN",
            Action::PhotomodeHeading => "PHOTOMODE_HEADING",
            Action::PhotomodePitch => "PHOTOMODE_PITCH",
            Action::PhotomodeEnableCameraMovement => "PHOTOMODE_ENABLE_CAMERA_MOVEMENT",
            Action::PhotomodeHideMenu => "PHOTOMODE_HIDE_MENU",
            Action::PhotomodeReset => "PHOTOMODE_RESET",
            Action::PhotomodeTakePhotoPc => "PHOTOMODE_TAKE_PHOTO_PC",
            Action::Pause => "PAUSE",
            Action::DebugEnable => "DEBUG_ENABLE",
            Action::DebugDisable => "DEBUG_DISABLE",
            Action::DebugSelectHidden => "DEBUG_SELECT_HIDDEN",
        }
        .into()
    }
}

impl Into<u32> for Action {
    fn into(self) -> u32 {
        match self {
            Action::MoveUp => 0,
            Action::MoveDown => 1,
            Action::MoveLeft => 2,
            Action::MoveRight => 3,
            Action::MovementX => 4,
            Action::MovementY => 5,
            Action::CameraX => 6,
            Action::CameraY => 7,
            Action::AbortVideo => 8,
            Action::AbortVideoConfirm => 9,
            Action::AbortTimeline => 10,
            Action::AbortTimelineConfirm => 0xb,
            Action::ScrollTimelinesUp => 0xc,
            Action::ScrollTimelinesDown => 0xd,
            Action::LoadingContinue => 0xe,
            Action::ToggleMenu => 0xf,
            Action::ToggleGameplayMenu => 0x10,
            Action::ToggleGameplayMenuDown => 0x11,
            Action::MenuA => 0x12,
            Action::MenuADown => 0x13,
            Action::MenuB => 0x14,
            Action::MenuBDown => 0x15,
            Action::MenuX => 0x16,
            Action::MenuXDown => 0x17,
            Action::MenuY => 0x18,
            Action::MenuYDown => 0x19,
            Action::MenuStart => 0x1a,
            Action::MenuBack => 0x1b,
            Action::MenuLeftthumb => 0x1c,
            Action::MenuRightthumb => 0x1d,
            Action::MenuLeftshoulder => 0x1e,
            Action::MenuRightshoulder => 0x1f,
            Action::MenuPadup => 0x20,
            Action::MenuPaddown => 0x21,
            Action::MenuPadleft => 0x22,
            Action::MenuPadright => 0x23,
            Action::MenuLeftthumbx => 0x24,
            Action::MenuLeftthumby => 0x25,
            Action::MenuRightthumbx => 0x26,
            Action::MenuRightthumby => 0x27,
            Action::MenuLefttrigger => 0x28,
            Action::MenuRighttrigger => 0x29,
            Action::PresentationLeft => 0x2a,
            Action::PresentationRight => 0x2b,
            Action::FreecameraForward1 => 0x2c,
            Action::FreecameraForward2 => 0x2d,
            Action::FreecameraBackward1 => 0x2e,
            Action::FreecameraBackward2 => 0x2f,
            Action::FreecameraX => 0x30,
            Action::FreecameraZ => 0x31,
            Action::FreecameraHeading => 0x32,
            Action::FreecameraPitch => 0x33,
            Action::FreecameraUp => 0x34,
            Action::FreecameraDown => 0x35,
            Action::FreecameraFaster => 0x36,
            Action::FreecameraSlower => 0x37,
            Action::FreecameraScaleTranslationUp => 0x38,
            Action::FreecameraScaleTranslationDown => 0x39,
            Action::FreecameraScaleRotationUp => 0x3a,
            Action::FreecameraScaleRotationDown => 0x3b,
            Action::FreecameraZoomIn => 0x3c,
            Action::FreecameraZoomOut => 0x3d,
            Action::FreecameraLensPrev => 0x3e,
            Action::FreecameraLensNext => 0x3f,
            Action::PhotomodeX => 300,
            Action::PhotomodeZ => 0x12d,
            Action::PhotomodeUp => 0x12e,
            Action::PhotomodeDown => 0x12f,
            Action::PhotomodeHeading => 0x130,
            Action::PhotomodePitch => 0x131,
            Action::PhotomodeEnableCameraMovement => 0x132,
            Action::PhotomodeHideMenu => 0x133,
            Action::PhotomodeReset => 0x134,
            Action::PhotomodeTakePhotoPc => 0x135,
            Action::Pause => 0x40,
            Action::DebugEnable => 0x41,
            Action::DebugDisable => 0x42,
            Action::DebugSelectHidden => 0x43,
        }
    }
}
