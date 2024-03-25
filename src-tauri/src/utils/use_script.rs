use crate::global_vars;
use crate::utils::run_command::run_command;
use prettytable::{Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use tauri::App;

#[derive(Debug, Serialize, Deserialize)]
struct PluginInfo {
    plugin_name: String,
    plugin_path: String,
}

pub fn save_plugin_info(app: &mut App, plugin_infos: &[PluginInfo]) {
    if let Some(cache_dir) = app.path_resolver().app_config_dir() {
        let file_path = cache_dir.join("cache/plugins_cache.json");

        let json_data = serde_json::to_string_pretty(&plugin_infos).unwrap();

        if let Ok(mut file) = File::create(&file_path) {
            if let Err(err) = file.write_all(json_data.as_bytes()) {
                eprintln!("Failed to write plugin info to file: {}", err);
            } else {
                println!("Plugin info saved to: {}", file_path.display());
            }
        } else {
            eprintln!("Failed to create file: {}", file_path.display());
        }
    } else {
        eprintln!("Cache directory not found");
    }
}

pub fn list_available_scripts(app: &mut App) {
    let mut plugin_infos = Vec::new(); // Creating a vtor for store plugin info

    if let Some(config_dir) = app.path_resolver().app_config_dir() {
        let plugins_dir = config_dir.join("plugins");

        if let Ok(entries) = fs::read_dir(&plugins_dir) {
            let mut table = Table::new();
            table.add_row(Row::new(vec![Cell::new("Plugins"), Cell::new("State")]));

            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(folder_name) = entry.file_name().to_str() {
                            let plugin_name = folder_name.to_string(); // Plugin name is the folder name
                            let plugin_path = entry.path().display().to_string(); // Full path to plugin folder
                                                                                  // Search by anyone file in the plugins folder with contains the plugin name
                            if let Ok(files) = fs::read_dir(entry.path()) {
                                for file in files.flatten() {
                                    if let Some(file_name) = file.file_name().to_str() {
                                        if file_name.contains(&plugin_name) {
                                            // Build the file path to script match of plugin
                                            let script_path = file.path().display().to_string();

                                            // Add the infos of plugin to PluginInfo list
                                            plugin_infos.push(PluginInfo {
                                                plugin_name: plugin_name.clone(),
                                                plugin_path: script_path.clone(),
                                            });

                                            // I need replace the loaded string to test results of
                                            // your plugin

                                            table.add_row(Row::new(vec![
                                                Cell::new(&plugin_name),
                                                Cell::new("✅"),
                                            ]));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            table.printstd();
        } else {
            eprintln!("Failed to read directory: {}", plugins_dir.display());
            return; // If fail, return of function to evite save empty file
        }
    } else {
        println!("Config directory not found");

        return; // If not find the config directory, return to avoid save empty file
    }

    // Save plugin info of files how JSON file
    save_plugin_info(app, &plugin_infos);
}

fn read_script_cache() -> io::Result<Vec<PluginInfo>> {
    let mut config_dir = global_vars::CONFIG_DIR.lock().unwrap();
    let file_path = config_dir.join("cache/plugins_cache.json");

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let cached_plugin_infos: Vec<PluginInfo> = serde_json::from_str(&contents)?;

    Ok(cached_plugin_infos)
}

fn find_plugin_path_by_name(
    plugin_name: &str,
    cached_plugin_infos: &[PluginInfo],
) -> Option<String> {
    for info in cached_plugin_infos {
        if info.plugin_name == plugin_name {
            return Some(info.plugin_path.clone());
        }
    }
    None
}

pub fn run_script(script_name: &str, command: &str) -> Result<String, String> {
    let cached_plugin_infos = match read_script_cache() {
        Ok(infos) => infos,
        Err(err) => {
            eprintln!("Failed to parse cache file: {}", err);
            return Err("Failed to parse cache file".to_string());
        }
    };

    let plugin_path = match find_plugin_path_by_name(script_name, &cached_plugin_infos) {
        Some(plugin_path) => {
            // println!("Plugin path for {} found: {}", script_name, plugin_path);
            Some(plugin_path) // Store value in variable
        }
        None => {
            eprintln!("Plugin path for {} not found", script_name);
            return Err("".to_string());
        }
    };

    let full_command = format!("{} {}", plugin_path.unwrap(), command);
    let result = run_command(&full_command);

    Ok(result)
}
