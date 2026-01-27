use std::num::{NonZero, NonZeroU32};
use std::os::macos::raw;

use glutin::display::{GlDisplay, GetGlDisplay};
use glutin::config::GlConfig;
use glutin::{context::{ContextAttributes, ContextAttributesBuilder}, prelude::NotCurrentGlContext};
use glutin::context::{NotCurrentContext, PossiblyCurrentContext};
use glutin::surface::{GlSurface, SurfaceTypeTrait};

use tauri_runtime::window::WindowId;
use tauri_runtime::UserEvent;

use tauri_runtime_wry::tao::window::WindowId as TaoWindowId;
use tauri_runtime_wry::EventLoopIterationContext;

/// Gets the WindowId from its TaoWindowId
pub(crate) fn get_id_from_tao_id<T: UserEvent>(
    tao_id: &TaoWindowId,
    context: &EventLoopIterationContext<'_, T>,
) -> Option<WindowId> {
    context.window_id_map.get(tao_id)
}

/// Gets the label of a Tauri window from its TaoWindowId
pub(crate) fn get_label_from_tao_id<T: UserEvent>(
    tao_id: &TaoWindowId,
    context: &EventLoopIterationContext<'_, T>,
) -> Option<String> {
    get_id_from_tao_id(tao_id, context).and_then(|id| {
        context
            .windows
            .0
            .borrow()
            .get(&id)
            .map(|ww| ww.label().to_string())
    })
}

pub fn get_raw_window_handle(window: &tauri::Window) -> raw_window_handle::RawWindowHandle {
    let ns_view= window.ns_view().unwrap();
    let ns_view_ptr = unsafe { core::ptr::NonNull::new_unchecked(ns_view) };
    let raw_w_handle = raw_window_handle::RawWindowHandle::AppKit(raw_window_handle::AppKitWindowHandle::new(ns_view_ptr)); 
    raw_w_handle
}

pub fn gen_display() -> glutin::display::Display {
    let raw_display_handle = raw_window_handle::RawDisplayHandle::AppKit(raw_window_handle::AppKitDisplayHandle::new());
    let gl_display = match unsafe { glutin::display::Display::new(
        raw_display_handle,
        glutin::display::DisplayApiPreference::Cgl,
    ) } {
        Ok(display) => display,
        Err(e) => {
            panic!("failed to create display: {}", e);
        }
    };
    gl_display
}

// Find the config with the maximum number of samples, so our triangle will be
// smooth.
pub fn gl_config_picker(configs: Box<dyn Iterator<Item = glutin::config::Config> + '_>) -> glutin::config::Config{
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

pub struct WindowGlowContext {
    pub gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
}


/*
pub fn change_gl_context(
    current_gl_context: &mut Option<glutin::context::PossiblyCurrentContext>,
    not_current_gl_context: &mut Option<glutin::context::NotCurrentContext>,
    gl_surface: &glutin::surface::Surface<glutin::surface::WindowSurface>,
) {

    if !cfg!(target_os = "windows") {
        // According to https://github.com/emilk/egui/issues/4289
        // we cannot do this early-out on Windows.
        // TODO(emilk): optimize context switching on Windows too.
        // See https://github.com/emilk/egui/issues/4173

        if let Some(current_gl_context) = current_gl_context {
            if gl_surface.is_current(current_gl_context) {
                return; // Early-out to save a lot of time.
            }
        }
    }

    let not_current = if let Some(not_current_context) = not_current_gl_context.take() {
        not_current_context
    } else {
        current_gl_context
            .take()
            .unwrap()
            .make_not_current()
            .unwrap()
    };

    *current_gl_context = Some(not_current.make_current(gl_surface).unwrap());
}
 */