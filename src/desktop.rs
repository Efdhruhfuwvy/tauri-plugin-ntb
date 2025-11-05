use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Ntb<R>> {
    Ok(Ntb(app.clone()))
}

/// Access to the ntb APIs.
pub struct Ntb<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Ntb<R> {}
