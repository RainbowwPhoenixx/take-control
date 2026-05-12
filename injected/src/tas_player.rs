use std::sync::{LazyLock, Mutex, MutexGuard};

use tracing::{error, info};
use windows::Win32::Foundation::{LPARAM, WPARAM};

use crate::{
    memory::app::{handle_keydown, handle_keyup},
    script::Script,
    windows_types::VirtualKeyCode,
};

#[derive(PartialEq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,

    // This state is only used for the tas controller interface
    Skipping,
}

#[derive(Default)]
pub struct HalfControllerState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub running: bool,

    pub mouse_pos: (i32, i32),
    pub left_click: bool,
    pub right_click: bool,
}

#[derive(Default)]
pub struct ControllerState {
    pub current: HalfControllerState,
    pub previous: HalfControllerState,
}

static TAS_PLAYER_SINGLETON: LazyLock<Mutex<TasPlayer>> =
    LazyLock::new(|| Mutex::new(TasPlayer::new()));

pub struct TasPlayer {
    pub state: PlaybackState,

    current_tick: u32,
    pub skipto_tick: u32,
    pub pauseat_tick: u32,

    next_line: usize,
    script_name: String,
    script: Option<Script>,

    controller: ControllerState,
}

impl TasPlayer {
    /// Creates a new TasPlayer
    fn new() -> Self {
        Self {
            state: PlaybackState::Stopped,
            current_tick: 0,
            skipto_tick: 0,
            pauseat_tick: 0,
            next_line: 0,
            script_name: "".to_string(),
            script: None,
            controller: Default::default(),
        }
    }

    /// Get the tas player singleton
    pub fn get() -> MutexGuard<'static, TasPlayer> {
        TAS_PLAYER_SINGLETON.lock().unwrap()
    }

    /// Starts the TAS
    /// If file is None, replay the current tas
    pub fn start(&mut self, file: Option<String>) {
        self.stop();

        if let Some(file) = file {
            self.script_name = file;
        }

        let path = if let Some(':') = self.script_name.chars().nth(1) {
            self.script_name.clone()
        } else {
            "./tas/".to_owned() + &self.script_name
        };

        tracing::info!("Reading tas file: {path}");
        self.script = match std::fs::read_to_string(path) {
            Err(err) => {
                error!("{err}");
                None
            }
            Ok(src) => match Script::try_from(src) {
                Err(parse_errs) => {
                    for err in &parse_errs {
                        error!("Parse error: {err}");
                    }
                    None
                }
                Ok(script) => Some(script),
            },
        };

        // Exit if no valid script
        let Some(script) = &self.script else { return };

        match &script.start {
            crate::script::StartType::Now => {}
        }

        // Reset variables
        self.controller = Default::default();
        self.current_tick = 0;
        self.next_line = 0;
        self.state = PlaybackState::Playing;

        info!("Started TAS")
    }

    pub fn resume(&mut self) {
        self.state = PlaybackState::Playing;
    }

    /// Stops the TAS
    pub fn stop(&mut self) {
        if self.state != PlaybackState::Stopped {
            self.state = PlaybackState::Stopped;

            let ticks = self.current_tick;
            info!("Stopped TAS after {ticks} ticks.")
        }
    }

    pub fn update_inputs(&mut self) {
        if self.state == PlaybackState::Stopped {
            return;
        }

        let Some(script) = self.script.as_ref() else {
            return;
        };

        if self.next_line >= script.lines.len() {
            self.stop();
            return;
        }

        let next_line = &script.lines[self.next_line];
        if next_line.tick == self.current_tick {
            self.next_line += 1;
            for key in &next_line.keys {
                let (down, code) = match key {
                    'W' => (true, VirtualKeyCode::W),
                    'A' => (true, VirtualKeyCode::A),
                    'S' => (true, VirtualKeyCode::S),
                    'D' => (true, VirtualKeyCode::D),
                    'w' => (false, VirtualKeyCode::W),
                    'a' => (false, VirtualKeyCode::A),
                    's' => (false, VirtualKeyCode::S),
                    'd' => (false, VirtualKeyCode::D),
                    _ => panic!("Unknown key"),
                };

                if down {
                    let window_ptr = crate::memory::app::G_WINDOW_PTR.resolve() as *const usize;
                    unsafe {
                        handle_keydown.call(
                            window_ptr.read(),
                            true,
                            WPARAM(code as usize),
                            LPARAM(0),
                        )
                    }
                } else {
                    unsafe { handle_keyup.call(0, code as u64) }
                }
            }
        }

        self.current_tick += 1;
        if self.current_tick == self.pauseat_tick {
            self.state = PlaybackState::Paused
        }
    }

    pub fn should_block_user_inputs(&self) -> bool {
        self.state != PlaybackState::Stopped
    }
}
