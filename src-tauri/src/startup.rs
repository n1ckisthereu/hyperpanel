use tauri::{App, AppHandle, Result};

//use crate::events::theme_listener::theme_listener;

fn start_events(app: AppHandle) -> Result<()> {
    //let _ = theme_listener(app);
    Ok(())
}

pub fn startup(app: &mut App) {
    let _ = start_events(app.handle());
}
