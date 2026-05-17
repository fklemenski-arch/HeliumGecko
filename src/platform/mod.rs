// src/platform/mod.rs
pub(crate) use platform_impl::*;

#[cfg(target_os = "windows")]
mod platform_impl {
    use winit::window::Window;
    use winit::platform::windows::WindowBuilderExtWindows;
    use crate::config::Config;
    pub fn apply_window_extras(builder: WindowBuilder, config: &Config) -> WindowBuilder {
        use windows::Win32::Graphics::Dwm::{DwmExtendFrameIntoClientArea, MARGINS};
        // Transparent window requires DwmExtendFrame
        if config.enable_transparency {
            // This will be applied after window creation – we just set the builder
            builder.with_transparent(true)
        } else {
            builder
        }
    }
    pub fn set_window_opacity(window: &Window, opacity: f32) {
        // Using Win32 API
        let hwnd = window.hwnd() as *mut _;
        unsafe {
            use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongW, GetWindowLongW, GWL_EXSTYLE, WS_EX_LAYERED, SetLayeredWindowAttributes, LWA_ALPHA};
            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED as i32);
            SetLayeredWindowAttributes(hwnd, 0, (opacity * 255.0) as u8, LWA_ALPHA);
        }
    }
}

#[cfg(target_os = "linux")]
mod platform_impl { ... }   // similar using X11
#[cfg(target_os = "macos")]
mod platform_impl { ... }   // similar using NSWindow
