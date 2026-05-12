#![allow(unused)]
#![warn(unused_imports)]

use std::cell::LazyCell;

use windows::{
    Win32::System::{
        LibraryLoader::{GetModuleHandleW, GetProcAddress},
        ProcessStatus::{GetModuleInformation, MODULEINFO},
        Threading::GetCurrentProcess,
    },
    core::{PCSTR, PCWSTR},
};

pub mod app;
pub mod control;
pub mod coregame;
pub mod input;
pub mod net;
pub mod physics;
pub mod renderer;
pub mod rl;

#[allow(missing_abi)]
unsafe extern "C" {
    #[link_name = "llvm.returnaddress"]
    fn return_address(a: i32) -> *const u8;
}

#[macro_export]
macro_rules! caller_address {
    () => {
        unsafe { crate::memory::return_address(0) }
    };
}

/// Methods for finding an address
pub enum AddressLocation {
    Address {
        module_name: String,
        offset: usize,
    },
    Pointerchain {
        base: usize,
        offsets: Vec<isize>,
    },
    PatternScan {
        module_name: String,
        pattern: String,
    },
    ModuleExport {
        module_name: String,
        symbol: String,
    },
}

impl AddressLocation {
    /// Resolve the address location into an address
    pub fn resolve(&self) -> usize {
        match self {
            AddressLocation::Address {
                module_name,
                offset,
            } => Self::get_module_base(&module_name) + offset,
            AddressLocation::ModuleExport {
                module_name,
                symbol,
            } => Self::get_module_symbol_address(module_name, symbol),
            AddressLocation::PatternScan {
                module_name,
                pattern,
            } => Self::get_module_pattern_address(&module_name, &pattern),
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

    fn get_module_pattern_address(module_str: &str, pattern_str: &str) -> usize {
        let info = Self::get_module_info(module_str);

        // Construct slice with the module info
        let haystack = unsafe {
            std::slice::from_raw_parts(info.lpBaseOfDll as *const u8, info.SizeOfImage as usize)
        };

        // Scan for the pattern
        let mut found_addr = None;
        let base_addr = info.lpBaseOfDll as usize;
        aobscan::PatternBuilder::from_ida_style(pattern_str)
            .unwrap()
            .build()
            .scan(haystack, |addr| {
                found_addr = Some(base_addr + addr);
                false
            });

        found_addr.expect(&format!(
            "Failed to find pattern [{pattern_str}] in module {module_str}"
        ))
    }

    fn get_module_info(module_str: &str) -> MODULEINFO {
        let module = module_str
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>();

        // Get module handle
        let handle = unsafe {
            GetModuleHandleW(PCWSTR(module.as_ptr() as _))
                .expect(&format!("Failed to find module {module_str}"))
        };

        // Get module info
        let mut info = MODULEINFO {
            lpBaseOfDll: std::ptr::null_mut(),
            SizeOfImage: 0,
            EntryPoint: std::ptr::null_mut(),
        };

        unsafe {
            GetModuleInformation(
                GetCurrentProcess(),
                handle,
                &mut info as *mut _,
                std::mem::size_of::<MODULEINFO>() as u32,
            )
        };

        info
    }

    fn get_module_base(module_str: &str) -> usize {
        Self::get_module_info(module_str).lpBaseOfDll as usize
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
                // TODO: make resolve return an error instead of panicking
                let address = std::panic::catch_unwind( || addr_location.resolve() );

                match address {
                    Err(e) => {
                        error!("Could not init hook {name}: panic while resolving address");
                        succeed = false;
                    }
                    Ok(addr) => {
                        // init
                        if let Err(e) =
                            unsafe { $hook.initialize(std::mem::transmute(addr), $detour) }
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
                    }
                }
            )*
            succeed
        }
    };
}

/// Creates a closure for detouring that logs the input arguments then calls the original function and logs the result
#[macro_export]
macro_rules! log_call {
    ( $hook:expr, $( $args:ident ),* ) => {
        |$($args),*| {
            debug!("{} called with args: {:?}", stringify!($hook), ($($args),*));
            let res = $hook.call($($args),*);
            debug!("{} returned: {:?}", stringify!($hook), res);
            res
        }
    };
}

/// Creates a closure for detouring that logs the input arguments then calls the original function and logs the result
#[macro_export]
macro_rules! transparent_call {
    ( $hook:expr, $( $args:ident ),* ) => {
        |$($args),*| {
            let res = $hook.call($($args),*);
            res
        }
    };
}

#[derive(Clone, Copy)]
pub enum DxVersion {
    Dx11,
    Dx12,
}

pub const DX_VERSION: LazyCell<DxVersion> = LazyCell::new(|| DxVersion::get());

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
pub fn init() {
    input::init();
    renderer::init();
    physics::init();
    coregame::init();
    net::init();
    rl::init();
    app::init();
    control::init();
}
