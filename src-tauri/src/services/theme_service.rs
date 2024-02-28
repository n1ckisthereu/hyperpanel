#[tauri::command]
pub fn get_pictures() {
    println!("I was invoked from JS!");
}
