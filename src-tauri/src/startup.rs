use tauri::{App, AppHandle, Result};

//use crate::events::theme_listener::theme_listener;
use crate::utils::use_script::list_available_scripts;

fn start_events(app: AppHandle) -> Result<()> {
    //let _ = theme_listener(app);
    Ok(())
}

pub fn startup(app: &mut App) {
    let _ = start_events(app.handle());
    let _ = list_available_scripts(app);
}
