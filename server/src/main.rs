// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use engine::Engine;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::thread;
use tracing::Level;
use yolo::Model;
mod chess;
mod common;
pub mod config;
mod engine;
pub mod listen;
pub mod logger;
mod worker;
pub mod yolo;

// 全局共享状态，用Arc和Mutex包装以实现线程安全共享
struct SharedState {
    config: Option<config::Config>,
    engine: Option<Engine>,
    model: Option<Model>,
    listen_thread: Option<thread::JoinHandle<()>>,
}

lazy_static! {
    // 使用lazy_static来创建一个全局的、可变的、线程安全的单例
    static ref STATE: Arc<Mutex<SharedState>> = Arc::new(Mutex::new(SharedState {
        config: None,
        engine: None,
        model: None,
        listen_thread: None,
    }));
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let lib_path = app.path_resolver().resolve_resource("../libs").unwrap();
            let config = config::Config::load(&lib_path);

            match config.loglevel.as_str() {
                "trace" => logger::init_tracer(Level::TRACE),
                "debug" => logger::init_tracer(Level::DEBUG),
                "info" => logger::init_tracer(Level::INFO),
                "warn" => logger::init_tracer(Level::WARN),
                "silent" => {}
                _ => logger::init_tracer(Level::ERROR),
            };
            let mut state = STATE.lock().unwrap();
            // 初始化引擎和模型
            let mut engine = engine::Engine::new(&lib_path);
            engine.set_chessdb(config.enable_chessdb);
            engine.set_show_wdl(config.show_wdl);
            engine.set_hash(config.engine_hash);
            engine.set_threads(config.engine_threads);
            state.engine = Some(engine);
            state.model = Some(yolo::Model::new(&lib_path).unwrap());
            state.config = Some(config);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            worker::start_listen,
            worker::stop_listen,
            config::set_engine_depth,
            config::set_engine_time,
            config::set_engine_threads,
            config::set_engine_hash,
            config::get_engine_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
