use hudhook::*;
use physx::prelude::Controller;

use crate::memory::physics::character_controller::CONTROLLER;

pub struct MyRenderLoop;

impl ImguiRenderLoop for MyRenderLoop {
    fn render(&mut self, ui: &mut imgui::Ui) {
        ui.window("Control TAS tool")
            .position([0., 0.], imgui::Condition::FirstUseEver)
            .size([320., 200.], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("hi :3");

                if let Some(capsule_controller) =
                    unsafe { CONTROLLER.as_ref().and_then(|c| c.px_controller.as_ref()) }
                {
                    let pos = capsule_controller.get_position();
                    ui.text(format!("pos: {:.3} {:.3} {:.3}", pos.x(), pos.y(), pos.z()));

                    let tmp = unsafe { &CONTROLLER.as_ref().and_then(|c| c.character_controller_state.as_ref()).unwrap().transfrom };
                    ui.text(format!("state: {tmp:#?}"));
                }
            });
    }
}
