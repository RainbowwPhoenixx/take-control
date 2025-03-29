#![feature(panic_update_hook)]
#![allow(static_mut_refs)]

use hudhook::hooks::{dx11::ImguiDx11Hooks, dx12::ImguiDx12Hooks};
use tracing::{error, info};

mod common_game_types;
mod gui;
mod memory;

#[unsafe(no_mangle)]
pub unsafe extern "stdcall" fn DllMain(
    hmodule: hudhook::windows::Win32::Foundation::HINSTANCE,
    reason: u32,
    _: *mut std::ffi::c_void,
) {
    if reason == hudhook::windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH {
        // Add hud
        hudhook::tracing::trace!("DllMain()");
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(1));

            init_logger();
            info!("Logger started.");

            info!("Initializing crash and panic handlers");
            init_panic_hook();
            init_exception_handler();

            let dx_version = memory::DxVersion::get();
            info!("Initializing hud.");
            init_hudhook(hmodule, dx_version);

            info!("Initializing memory hooks.");
            memory::init(dx_version);
        });
    }
}

/// Attempt to initialize hudhook for the given hook type.
///
/// Returns true on success
fn init_hudhook(
    hmodule: windows::Win32::Foundation::HINSTANCE,
    dx_version: memory::DxVersion,
) -> bool {
    let status = match dx_version {
        memory::DxVersion::Dx11 => {
            hudhook::Hudhook::builder().with::<ImguiDx11Hooks>(gui::MyRenderLoop)
        }
        memory::DxVersion::Dx12 => {
            hudhook::Hudhook::builder().with::<ImguiDx12Hooks>(gui::MyRenderLoop)
        }
    }
    .with_hmodule(hmodule)
    .build()
    .apply();

    if let Err(e) = status {
        hudhook::tracing::error!("Couldn't apply hooks for {dx_version}: {e:?}");
        hudhook::eject();
        return false;
    }

    true
}

fn init_logger() {
    let file_appender = tracing_appender::rolling::never(
        "C:\\Users\\rainbow\\Documents\\control-tas",
        "control_tas.log",
    );
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_target(false)
        .init();

    // If we don't do this, the logger dies at the end of this thread.
    // We want it to live for our hooks even when the init is done.
    std::mem::forget(_guard);
}

fn init_panic_hook() {
    std::panic::update_hook(move |prev, info| {
        if let Some(location) = info.location() {
            error!(
                "TAS tool panicked in file {} at line {}: {info}",
                location.file(),
                location.line()
            );
        } else {
            error!("TAS tool panicked: {info}")
        }

        prev(info)
    });
}

// Just to log control crashes
fn init_exception_handler() {
    use windows::Win32::{
        Foundation::{EXCEPTION_ACCESS_VIOLATION, EXCEPTION_STACK_OVERFLOW},
        System::{
            Diagnostics::Debug::{
                EXCEPTION_EXECUTE_HANDLER, EXCEPTION_POINTERS, MAX_SYM_NAME,
                RtlCaptureStackBackTrace, SYMBOL_INFO, SymCleanup, SymFromAddr, SymInitialize,
            },
            Threading::GetCurrentProcess,
        },
    };
    
    unsafe extern "system" fn handler(exceptioninfo: *mut EXCEPTION_POINTERS) -> i32 {
        let exception_record = unsafe {
            exceptioninfo
                .as_ref()
                .unwrap()
                .ExceptionRecord
                .as_ref()
                .unwrap()
        };

        let cause = match exception_record.ExceptionCode {
            EXCEPTION_ACCESS_VIOLATION | EXCEPTION_STACK_OVERFLOW => "Segmentation fault",
            _ => return EXCEPTION_EXECUTE_HANDLER,
        };

        let mut buf = String::new();
        buf += &format!("{cause} at {:p}\n", exception_record.ExceptionAddress);

        // unwind the stack and log it
        let process = unsafe { GetCurrentProcess() };

        if let Err(e) =
            unsafe { SymInitialize(process, windows::core::PCSTR(std::ptr::null()), true) }
        {
            buf += &format!(
                "Failed to init symbols ({e:?}), crash report might look uglier than usual"
            );
        }

        let stack = &mut [std::ptr::null_mut(); 100];
        let frame_count = unsafe { RtlCaptureStackBackTrace(0, stack, None) } as usize;

        for (frame_idx, &addr) in stack[0..frame_count].iter().enumerate() {
            // This API is a joke. we have to do this fuckery to get a `SYMBOL_INFO` struct followed
            // by a char buffer, cause that's what the API wants. Get your shit together windows.
            let symbol_buf =
                &mut [0_u8; size_of::<SYMBOL_INFO>() + MAX_SYM_NAME as usize * size_of::<char>()];
            let symbol: &mut SYMBOL_INFO = unsafe { std::mem::transmute(symbol_buf as *mut _) };
            symbol.SizeOfStruct = size_of::<SYMBOL_INFO>() as u32;
            symbol.MaxNameLen = MAX_SYM_NAME;

            let got_symbol = unsafe { SymFromAddr(process, addr as u64, None, symbol as _) };
            let symbol_name = match got_symbol {
                Ok(()) => {
                    if symbol.NameLen == 0 {
                        format!("{addr:p}")
                    } else {
                        unsafe { std::ffi::CStr::from_ptr(&symbol.Name as *const i8) }
                            .to_string_lossy()
                            .into_owned()
                    }
                }
                Err(_) => {
                    format!("{addr:p}")
                }
            };
            buf += &format!("\t{}: {symbol_name}\n", frame_count - frame_idx - 1);
        }

        unsafe {
            let _ = SymCleanup(process);
        };
        error!("{buf}");

        EXCEPTION_EXECUTE_HANDLER
    }

    unsafe {
        windows::Win32::System::Diagnostics::Debug::AddVectoredExceptionHandler(0, Some(handler))
    };
}
