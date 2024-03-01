use prettytable::{Cell, Row, Table};
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use tauri::App;

#[derive(Serialize)]
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
    let mut plugin_infos = Vec::new(); // Criando um vetor para armazenar as informações do plugin

    if let Some(config_dir) = app.path_resolver().app_config_dir() {
        let plugins_dir = config_dir.join("plugins");

        if let Ok(entries) = fs::read_dir(&plugins_dir) {
            let mut table = Table::new();
            table.add_row(Row::new(vec![Cell::new("Plugins"), Cell::new("State")]));

            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(folder_name) = entry.file_name().to_str() {
                            let plugin_name = folder_name.to_string(); // Nome do plugin é o nome da pasta
                            let plugin_path = entry.path().display().to_string(); // Caminho completo da pasta do plugin

                            // Procurar por qualquer arquivo dentro da pasta do plugin que contenha o nome do plugin
                            if let Ok(files) = fs::read_dir(entry.path()) {
                                for file in files.flatten() {
                                    if let Some(file_name) = file.file_name().to_str() {
                                        if file_name.contains(&plugin_name) {
                                            // Construir o caminho do script correspondente ao plugin
                                            let script_path = file.path().display().to_string();

                                            // Adicionando as informações do plugin à lista de PluginInfo
                                            plugin_infos.push(PluginInfo {
                                                plugin_name: plugin_name.clone(),
                                                plugin_path: script_path.clone(),
                                            });

                                            // Você deve substituir a string "loaded" pelo estado real do plugin
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
            return; // Se falhar, retorne da função para evitar salvar um arquivo vazio
        }
    } else {
        println!("Config directory not found");
        return; // Se não encontrar o diretório de configuração, retorne da função para evitar salvar um arquivo vazio
    }

    // Salvar informações do plugin como arquivo JSON
    save_plugin_info(app, &plugin_infos);
}
