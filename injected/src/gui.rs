use hudhook::*;

use crate::memory::input::input_manager;

pub struct MyRenderLoop;

impl ImguiRenderLoop for MyRenderLoop {
    fn render(&mut self, ui: &mut imgui::Ui) {
        ui.window("Control TAS tool")
            .position([0., 0.], imgui::Condition::FirstUseEver)
            .size([320., 200.], imgui::Condition::FirstUseEver)
            .build(|| {
                let im = unsafe { input_manager::getInstance.call().as_mut().unwrap() };
                ui.text(format!("{im:?}"));
            });
    }
}
