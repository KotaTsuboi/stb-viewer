#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::st_bridge::stb_model::stb_nodes::StbNode;
use app::st_bridge::StBridge;

#[tauri::command]
fn read_st_bridge(file_name: &str) -> StBridge {
    app::read_st_bridge(file_name)
}

#[tauri::command]
fn members(st_bridge: StBridge) -> Vec<(&'static StbNode, &'static StbNode)> {
    st_bridge.members()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![members])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
