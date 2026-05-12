use hudhook::*;

use crate::{
    memory::{input::input_manager, physics::character_controller::CONTROLLER},
    tas_player::TasPlayer,
};

pub struct TasToolGui {
    filename: String,
    tas_pause_at: u32,
}

impl TasToolGui {
    pub fn new() -> Self {
        Self {
            filename: "test.ctas".into(),
            tas_pause_at: 0,
        }
    }
}

impl ImguiRenderLoop for TasToolGui {
    fn render(&mut self, ui: &mut imgui::Ui) {
        ui.window("Control TAS tool")
            .position([0., 0.], imgui::Condition::FirstUseEver)
            .size([320., 200.], imgui::Condition::FirstUseEver)
            .build(|| {
                if ui.button("start TAS") {
                    TasPlayer::get().start(Some(self.filename.clone()));
                }

                if ui.button("resume TAS") {
                    TasPlayer::get().resume();
                }

                let _ = ui.input_text("Filename", &mut self.filename).build();
                if ui.input_scalar("Pause at", &mut self.tas_pause_at).build() {
                    TasPlayer::get().pauseat_tick = self.tas_pause_at;
                }

                let im = unsafe { input_manager::getInstance.call().as_mut().unwrap() };
                ui.text(format!("{im:#?}"));

                if let Some(controller) = unsafe { CONTROLLER.as_ref() } {
                    ui.text(format!("{controller:#?}"));
                }
            });
    }
}
