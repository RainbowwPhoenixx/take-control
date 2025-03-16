#![feature(panic_update_hook)]
#![allow(static_mut_refs)]

use hudhook::hooks::{dx11::ImguiDx11Hooks, dx12::ImguiDx12Hooks};
use tracing::{error, info};

mod gui;
mod memory;
mod common_game_types;

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

            init_panic_hook();

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
