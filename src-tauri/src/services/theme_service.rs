use std::fs;
use std::path::{Path, PathBuf};
use tauri::api::path::picture_dir;

fn list_images_path(directory_path: &Path) -> Vec<PathBuf> {
    // Reading of files in specified directory and filtering by images;
    fs::read_dir(directory_path)
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
        .collect()
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
