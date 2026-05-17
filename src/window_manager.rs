use std::sync::{Arc, Mutex};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowBuilder};
use winit::platform::windows::WindowBuilderExtWindows; // for transparency
use glutin_winit::DisplayBuilder;
use glutin::context::PossiblyCurrentContext;
use glutin::surface::Surface;
use raw_window_handle::HasRawWindowHandle;
use crate::config::Config;
use crate::session::{SessionState, WindowState};
use crate::engine::{ServoEngine, BrowserEngine};
use crate::overlay::OverlayUI;
use crate::platform;

pub struct ManagedWindow {
    window: Arc<Window>,
    gl_context: PossiblyCurrentContext,
    gl_surface: Surface<glutin::surface::WindowSurface>,
    engine: Box<dyn BrowserEngine>,
    overlay: OverlayUI,
    opacity: f32,
}

pub struct WindowManager {
    config: Config,
    session: Arc<Mutex<SessionState>>,
    event_loop: EventLoop<()>,
    windows: Vec<ManagedWindow>,
}

impl WindowManager {
    pub fn new(config: Config, session: Arc<Mutex<SessionState>>) -> Self {
        let event_loop = EventLoopBuilder::new().build();
        Self { config, session, event_loop, windows: Vec::new() }
    }

    pub fn open_window(&mut self, url: &str) {
        let window_builder = WindowBuilder::new()
            .with_title("Helium")
            .with_decorations(!self.config.frameless)
            .with_transparent(self.config.enable_transparency)
            .with_always_on_top(self.config.always_on_top)
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.config.window_width,
                self.config.window_height,
            ));

        // Platform-specific extensions
        let window_builder = platform::apply_window_extras(window_builder, &self.config);

        let window = window_builder.build(&self.event_loop).unwrap();
        let window = Arc::new(window);

        // Create GL context via glutin
        let (gl_context, gl_surface) = {
            let raw_window_handle = window.raw_window_handle();
            let display_builder = DisplayBuilder::new().with_window_handle(raw_window_handle);
            let (display, gl_config) = display_builder
                .build(&self.event_loop)
                .expect("Could not create gl display");
            let context = display.create_context(&gl_config, &glutin::context::ContextAttributesBuilder::new().build(None)).unwrap();
            let surface = display.create_window_surface(&gl_config, &window.raw_window_handle()).unwrap();
            (context, surface)
        };

        // Make context current
        let _ = gl_context.make_current(&gl_surface);

        // Initialise engine (Servo)
        let engine = ServoEngine::new(window.clone(), self.event_loop.create_proxy());

        // Set initial opacity
        let opacity = self.config.default_opacity;
        platform::set_window_opacity(&window, opacity);

        // Create overlay UI (egui)
        let overlay = OverlayUI::new(window.clone(), &gl_context);

        // Save state
        {
            let mut state = self.session.lock().unwrap();
            state.windows.push(WindowState {
                url: url.to_string(),
                opacity,
                x: window.outer_position().unwrap().x,
                y: window.outer_position().unwrap().y,
                width: self.config.window_width,
                height: self.config.window_height,
            });
        }

        let managed = ManagedWindow {
            window,
            gl_context,
            gl_surface,
            engine,
            overlay,
            opacity,
        };
        managed.engine.load_url(url);
        self.windows.push(managed);
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _target, control_flow| {
            *control_flow = ControlFlow::Poll;
            for win in &mut self.windows {
                // Handle events, compose and render...
            }
        });
    }
}
