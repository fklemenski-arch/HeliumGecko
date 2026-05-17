use winit::event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ModifiersState};
use crate::session::SessionState;
use std::sync::{Arc, Mutex};

pub struct ShortcutHandler {
    session: Arc<Mutex<SessionState>>,
}

impl ShortcutHandler {
    pub fn new(session: Arc<Mutex<SessionState>>) -> Self {
        Self { session }
    }

    pub fn process_event(
        &self,
        event: &Event<()>,
        quit: &mut bool,
        show_slider: &mut bool,
        next_url: &mut Option<String>,
    ) {
        if let Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(key), state, modifiers, .. }, .. }, .. } = event {
            if state.is_pressed() {
                match (modifiers, key) {
                    (ModifiersState::CTRL, VirtualKeyCode::Q) => *quit = true,
                    (ModifiersState::CTRL | ModifiersState::SHIFT, VirtualKeyCode::O) => *show_slider = true,
                    (ModifiersState::CTRL, VirtualKeyCode::N) => {
                        // new window: we signal by setting next_url to a copy of current URL
                        // handled in window_manager
                    }
                    _ => {}
                }
            }
        }
    }
}
