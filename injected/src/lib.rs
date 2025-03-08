#![feature(panic_update_hook)]
#![allow(static_mut_refs)]
use std::ffi::CString;
use std::iter;

use hudhook::hooks::dx11::ImguiDx11Hooks;
use hudhook::*;
use retour::static_detour;
use tracing::{error, info};
use windows::{
    Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress},
    core::{PCSTR, PCWSTR},
};

static mut DELTA_X: f32 = 0.0;

pub struct MyRenderLoop;

impl ImguiRenderLoop for MyRenderLoop {
    fn render(&mut self, ui: &mut imgui::Ui) {
        ui.window("My first render loop")
            .position([0., 0.], imgui::Condition::FirstUseEver)
            .size([320., 200.], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Hello, hello!");
                unsafe {
                    ui.text(format!("DeltaX: {DELTA_X}"));
                };
            });
    }
}

static_detour! {
    static GetMouseDeltaX: unsafe extern "system" fn(usize) -> f32;
}

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

            info!("Starting initialiser thread.");
            std::thread::sleep(std::time::Duration::from_secs(1));

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

            if let Err(e) = hudhook::Hudhook::builder()
                .with::<ImguiDx11Hooks>(MyRenderLoop)
                .with_hmodule(hmodule)
                .build()
                .apply()
            {
                hudhook::tracing::error!("Couldn't apply hooks: {e:?}");
                hudhook::eject();
            }

            // Hook functions
            let address =
                get_module_symbol_address("input_rmdwin7_f.dll", "?getMouseDeltaX@InputX86@input@@QEAAMXZ");
            info!(address);

            unsafe {
                GetMouseDeltaX
                    .initialize(std::mem::transmute(address), get_mouse_delta_x)
                    .unwrap()
                    .enable()
                    .unwrap()
            };
        });
    }
}

fn get_mouse_delta_x(this: usize) -> f32 {
    let dx = unsafe { GetMouseDeltaX.call(this) };
    unsafe { DELTA_X = dx };
    dx
}

fn get_module_symbol_address(module_str: &str, symbol_str: &str) -> usize {
    let module = module_str
        .encode_utf16()
        .chain(iter::once(0))
        .collect::<Vec<u16>>();
    let symbol = CString::new(symbol_str).unwrap();
    unsafe {
        let handle = GetModuleHandleW(PCWSTR(module.as_ptr() as _))
            .expect(&format!("Failed to find module {module_str}"));
        GetProcAddress(handle, PCSTR(symbol.as_ptr() as _)).expect(&format!(
            "Failed to find symbol {symbol_str} in module {module_str}"
        )) as usize
    }
}
