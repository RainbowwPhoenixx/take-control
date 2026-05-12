pub fn init() {}

pub mod game_update_object_state {
    use crate::memory::net::network_address::NetworkRole;

    use super::GlobalID;

    #[repr(C)]
    pub struct GameUpdateObjectState {
        vtable: usize,
        pub network_role: NetworkRole,
        _pad1: u32,
        pub network_update_state: u32,
        _pad2: u32,
        global_id: GlobalID,
    }
}

pub mod game_update_object {
    use super::game_update_object_state::GameUpdateObjectState;

    #[repr(C)]
    pub struct GameUpdateObject {
        vtable: usize,
        pub state: *mut GameUpdateObjectState,
        pub active_for_state: u32,
        pub ready_for_update: bool,
    }
}

pub mod component_base {
    use super::game_update_object::GameUpdateObject;

    pub struct ComponentBase {
        base: GameUpdateObject,
    }
}

pub mod component_state_base {
    use super::{component_base::ComponentBase, game_update_object_state::GameUpdateObjectState};

    #[repr(C)]
    pub struct ComponentStateBase {
        base: GameUpdateObjectState,
        component_base: *mut ComponentBase,
        game_object_state: usize,
    }
}

type GlobalID = u64;
