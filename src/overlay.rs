use egui::{Context, Slider, Button};
use egui_winit::State;
use egui_glow::Painter;
use glow::HasContext;
use std::sync::Arc;
use winit::window::Window;

pub struct OverlayUI {
    egui_state: State,
    painter: Painter,
    show_slider: bool,
}

impl OverlayUI {
    pub fn new(window: Arc<Window>, gl_context: &glutin::context::PossiblyCurrentContext) -> Self {
        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                gl_context.display().get_proc_address(s) as *const _
            })
        };
        let painter = Painter::new(Arc::new(gl), "", None).unwrap();
        let egui_state = State::new(window.raw_window_handle(), window.raw_display_handle(), window.scale_factor() as f32, None);
        Self { egui_state, painter, show_slider: false }
    }

    pub fn toggle_slider(&mut self) {
        self.show_slider = !self.show_slider;
    }

    pub fn render(
        &mut self,
        window: &Window,
        current_opacity: &mut f32,
        close_requested: &mut bool,
    ) -> Vec<egui::ClippedPrimitive> {
        let raw_input = self.egui_state.take_egui_input(window);
        let full_output = egui::CentralPanel::default().show(&raw_input, |ui| {
            ui.horizontal(|ui| {
                if ui.button("✖").clicked() {
                    *close_requested = true;
                }
                if self.show_slider {
                    ui.add(Slider::new(current_opacity, 0.1..=1.0).text("Opacity"));
                }
                if ui.button("⚙").clicked() {
                    self.show_slider = !self.show_slider;
                }
            });
        });
        self.egui_state.handle_platform_output(window, &full_output.platform_output);
        full_output.shapes
    }

    pub fn paint(&mut self, primitives: Vec<egui::ClippedPrimitive>, dimensions: [u32; 2]) {
        let mut pixels = egui::ColorImage::example(); // placeholder
        let textures_delta = std::mem::take(&mut self.egui_state.textures_delta);
        self.painter.paint_primitives(dimensions, self.egui_state.pixels_per_point, &primitives);
    }
}
