#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use dash_backend::app;
use tauri::{ Builder, async_runtime };

struct Port(u16);

fn main() {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");
    async_runtime::spawn(app(port));
    Builder::default()
        .manage(Port(port))
        .invoke_handler(tauri::generate_handler![get_port])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_port(port: tauri::State<Port>) -> Result<String, String> {
    Ok(format!("{}", port.0))
}
