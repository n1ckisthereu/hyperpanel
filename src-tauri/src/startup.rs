use tauri::{App, AppHandle, Result};

//use crate::events::theme_listener::theme_listener;
use crate::global_vars;
use crate::utils::use_script::list_available_scripts;

fn define_global_vars(app: &mut App) -> Result<()> {
    let mut config_dir = global_vars::CONFIG_DIR.lock().unwrap();

    if let Some(dir) = app.path_resolver().app_config_dir() {
        *config_dir = dir.clone();
    }

    Ok(())
}

fn start_events(app: AppHandle) -> Result<()> {
    //let _ = theme_listener(app);

    Ok(())
}

pub fn startup(app: &mut App) {
    let _ = define_global_vars(app);

    let _ = start_events(app.handle());
    let _ = list_available_scripts(app);
}
