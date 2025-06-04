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
use desktop::MediaSession;
#[cfg(mobile)]
use mobile::MediaSession;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the media-session APIs.
pub trait MediaSessionExt<R: Runtime> {
  fn media_session(&self) -> &MediaSession<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MediaSessionExt<R> for T {
  fn media_session(&self) -> &MediaSession<R> {
    self.state::<MediaSession<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("media-session")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let media_session = mobile::init(app, api)?;
      #[cfg(desktop)]
      let media_session = desktop::init(app, api)?;
      app.manage(media_session);
      Ok(())
    })
    .build()
}
