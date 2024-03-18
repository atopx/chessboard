// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod chess;
mod yolo;

const MODEL_PATH: &str = "../libs/model.onnx";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(app: tauri::AppHandle, name: &str) -> String {
    let resurce_path = app.path_resolver().resolve_resource(MODEL_PATH).unwrap();
    let model = yolo::Model::new(resurce_path).unwrap();
    let window = chess::utils::get_windows(name).unwrap();
    let image = window.capture_image().unwrap();
    let detections = model.predict(image).unwrap();
    let count = detections.len();
    println!("detections count {}", count);
    format!("detections count {}", count)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
