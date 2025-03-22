use windows::{
    Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress},
    core::{PCSTR, PCWSTR},
};

pub mod input;
pub mod renderer;
pub mod physics;

/// Methods for finding an address
pub enum AddressLocation {
    Address(usize),
    Pointerchain { base: usize, offsets: Vec<isize> },
    PatternScan(String),
    ModuleExport { module_name: String, symbol: String },
}

impl AddressLocation {
    /// Resolve the address location into an address
    fn resolve(&self) -> usize {
        match self {
            AddressLocation::ModuleExport {
                module_name,
                symbol,
            } => Self::get_module_symbol_address(module_name, symbol),
            _ => todo!(),
        }
    }
    /// Read a value from the address
    fn read(&self) {
        unimplemented!()
    }
    /// Write a value to the address
    fn write(&self) {
        unimplemented!()
    }

    fn get_module_symbol_address(module_str: &str, symbol_str: &str) -> usize {
        let module = module_str
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>();
        let symbol = std::ffi::CString::new(symbol_str).unwrap();
        unsafe {
            let handle = GetModuleHandleW(PCWSTR(module.as_ptr() as _))
                .expect(&format!("Failed to find module {module_str}"));
            GetProcAddress(handle, PCSTR(symbol.as_ptr() as _)).expect(&format!(
                "Failed to find symbol {symbol_str} in module {module_str}"
            )) as usize
        }
    }
}

/// Inits and enables a list of hooks.
///
/// Replaces the function at address with our function using the static detour.
/// Returns false if at least one of the hooks failed to initialize, true otherwise
///
/// # Examples
///
/// ```no_run
/// init_hook!(
///     static_detour1 @ address_location1 -> our_function1,
///     static_detour2 @ address_location2 -> our_function2,
/// );
/// ```
#[macro_export]
macro_rules! init_enable_hook {
    ( $( $hook: expr, @ $target_addr: ident -> $detour: expr );* $(;)? ) => {
        {
            use tracing::{error, debug};
            let mut succeed = true;
            $(
                let name = stringify!($hook);
                let addr_location: AddressLocation = $target_addr;
                // init
                if let Err(e) =
                    unsafe { $hook.initialize(std::mem::transmute(addr_location.resolve()), $detour) }
                {
                    error!("Failed to init hook {name}! {e}");
                    succeed = false;
                }

                // enable
                if let Err(e) = unsafe { $hook.enable() } {
                    error!("Failed to enable hook {name}! {e}");
                    succeed = false
                } else {
                    debug!("Enabled hook {name}")
                }
            )*
            succeed
        }
    };
}

#[derive(Clone, Copy)]
pub enum DxVersion {
    Dx11,
    Dx12,
}

impl DxVersion {
    /// Find out which version of the game we are running on and return it.
    pub fn get() -> Self {
        // if win7 version of the dll is loaded, we are on dx11, else on dx12
        let module = "input_rmdwin7_f.dll"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>();
        match unsafe { GetModuleHandleW(PCWSTR(module.as_ptr() as _)) } {
            Ok(_) => Self::Dx11,
            Err(_) => Self::Dx12,
        }
    }
}

impl std::fmt::Display for DxVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DxVersion::Dx11 => f.write_str("DirectX11"),
            DxVersion::Dx12 => f.write_str("DirectX12"),
        }
    }
}

/// Init all the hooks
pub fn init(dx_version: DxVersion) {
    input::init(dx_version);
    renderer::init(dx_version);
    physics::init(dx_version);
}
