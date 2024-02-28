use std::fs;
use std::path::{Path, PathBuf};
use tauri::api::path::picture_dir;

fn list_images_path(directory_path: &Path) -> Vec<PathBuf> {
    // Reading of files in specified directory and filtering by images;
    let mut images_paths: Vec<PathBuf> = fs::read_dir(directory_path)
        .unwrap_or_else(|_| panic!("Error on read directory: {:?}", directory_path))
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.metadata().map(|m| m.is_file()).unwrap_or(false) {
                    e.path().extension().and_then(|ext| {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if ["jpg", "jpeg", "png"].contains(&&*ext_str) {
                            Some(e.path())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    // Ordenar os caminhos dos arquivos de imagem
    images_paths.sort_by(|a, b| {
        let a_name = a.file_name().unwrap().to_string_lossy();
        let b_name = b.file_name().unwrap().to_string_lossy();
        extract_number(&a_name).cmp(&extract_number(&b_name))
    });

    images_paths
}

fn extract_number(s: &str) -> usize {
    s.chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0, |acc, digit| acc * 10 + digit as usize)
}

#[tauri::command]
pub fn get_pictures() {
    if let Some(mut picture_dir) = picture_dir() {
        picture_dir.push("Wallpapers");
        let images_paths = list_images_path(&picture_dir);
        for path in &images_paths {
            println!("{}", path.to_string_lossy());
        }
    }
}
