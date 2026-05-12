pub fn init() {}

pub mod network_address {
    use crate::common_game_types::InplaceString;

    #[derive(Debug)]
    #[repr(C)]
    pub struct NetworkAddress {
        ip: InplaceString<128>,
        port: u16,
    }

    #[derive(Debug)]
    #[repr(C)]
    pub enum NetworkRole {
        SERVER,
        CLIENT,
        UNK,
    }
}
