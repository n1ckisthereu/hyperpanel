use prettytable::{Cell, Row, Table};
use std::fs;
use tauri::App;

pub fn list_available_scripts(app: &mut App) {
    if let Some(config_dir) = app.path_resolver().app_config_dir() {
        let plugins_dir = config_dir.join("plugins");

        if let Ok(entries) = fs::read_dir(&plugins_dir) {
            let mut table = Table::new();
            table.add_row(Row::new(vec![Cell::new("Plugins"), Cell::new("State")]));

            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(folder_name) = entry.file_name().to_str() {
                            // You should replace the "loaded" string with the actual state of the plugin
                            table.add_row(Row::new(vec![Cell::new(folder_name), Cell::new("✅")]));
                        }
                    }
                }
            }

            table.printstd();
        } else {
            eprintln!("Failed to read directory: {}", plugins_dir.display());
        }
    } else {
        println!("Config directory not found");
    }
}
