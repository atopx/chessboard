// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use engine::Engine;
use lazy_static::lazy_static;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use yolo::Model;
mod chess;
mod common;
mod engine;
pub mod listen;
pub mod logger;
mod worker;
pub mod yolo;

// 全局共享状态，用Arc和Mutex包装以实现线程安全共享
struct SharedState {
    engine: Option<Engine>,
    model: Option<Model>,
    listen_thread: Option<thread::JoinHandle<()>>,
}

lazy_static! {
    // 使用lazy_static来创建一个全局的、可变的、线程安全的单例
    static ref STATE: Arc<Mutex<SharedState>> = Arc::new(Mutex::new(SharedState {
        engine: None,
        model: None,
        listen_thread: None,
    }));
}

fn main() {
    logger::init_tracer();

    tauri::Builder::default()
        .setup(|app| {
            let lib_path = app.path_resolver().resolve_resource("../libs").unwrap();
            let mut state = STATE.lock().unwrap();
            // 初始化引擎和模型
            state.engine = Some(engine::Engine::new(&lib_path));
            state.model = Some(yolo::Model::new(&lib_path).unwrap());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            worker::start_listen,
            worker::stop_listen,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
