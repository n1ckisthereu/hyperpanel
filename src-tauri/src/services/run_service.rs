use crate::utils::use_script::run_script;

#[tauri::command]
pub fn run(pname: &str, command: &str) -> String {
    run_script(pname, command).unwrap().into()
}
