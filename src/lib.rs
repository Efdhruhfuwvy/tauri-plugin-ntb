use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Ntb;
#[cfg(mobile)]
use mobile::Ntb;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the ntb APIs.
pub trait NtbExt<R: Runtime> {
    fn ntb(&self) -> &Ntb<R>;
}

impl<R: Runtime, T: Manager<R>> crate::NtbExt<R> for T {
    fn ntb(&self) -> &Ntb<R> {
        self.state::<Ntb<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("ntb")
        .invoke_handler(tauri::generate_handler![
            commands::minimize,
            commands::toggle_maximize,
            commands::close,
            commands::drag,
            commands::move_by,
            commands::get_drag_behavior,
            commands::is_maximized,
            commands::get_window_controls,
            commands::get_window_control_image,
            commands::get_title_bar_css,
            commands::double_click_title_bar,
            commands::right_click_title_bar,
            commands::middle_click_title_bar,
            commands::show_snap_overlay,
            commands::hide_snap_overlay,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let ntb = mobile::init(app, api)?;
            #[cfg(desktop)]
            let ntb = desktop::init(app, api)?;
            app.manage(ntb);
            Ok(())
        })
        .build()
}
