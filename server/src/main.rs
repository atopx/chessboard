// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

mod chess;
mod engine;
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
        .setup(|app| {
            let (tx, rx): (Sender<String>, Receiver<String>) = channel();
            // let mut chess_board = ChessBoard::new();

            thread::spawn(move || {
                loop {
                    {
                        // todo 重复判断棋盘是否发生变化:
                        //   - 是 => 发送move事件到前端
                        //   - 是否需要分析:
                        //     - 是 => 发送fen到分析队列
                        unimplemented!();
                    }
                }
            });

            thread::spawn(move || loop {
                // 监听分析队列，收到事件后:
                //   调用云库查询, 是否查询成功
                //     - 是 => 是否满足阈值设置:
                //       - 是 => 发送日志到前端
                //       - 否 => 调用引擎 => 发送日志到前端
                //     - 否 => 调用引擎 => 发送日志到前端

                match rx.recv() {
                    Ok(message) => {}
                    Err(e) => {}
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
