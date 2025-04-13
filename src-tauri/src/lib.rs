use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use engine::Engine;
use lazy_static::lazy_static;

mod chess;
mod common;
mod config;
mod engine;
mod listen;
mod logger;
mod worker;
mod yolo;

// 全局共享状态，用Arc和Mutex包装以实现线程安全共享
struct SharedState {
    config: Option<config::Config>,
    engine: Option<Engine>,
    listen_thread: Option<thread::JoinHandle<()>>,
}

lazy_static! {
    static ref STATE: Arc<Mutex<SharedState>> =
        Arc::new(Mutex::new(SharedState { config: None, engine: None, listen_thread: None }));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logger::init_tracer(tracing::Level::DEBUG);
    tauri::Builder::default()
        .setup(|_app| {
            let lib_path = Path::new("../libs");
            let config = config::Config::load(lib_path);
            let mut engine = engine::Engine::new(lib_path);
            engine.set_chessdb(config.enable_chessdb);
            engine.set_show_wdl(config.show_wdl);
            engine.set_hash(config.engine_hash);
            engine.set_threads(config.engine_threads);
            let mut state = STATE.lock().unwrap();
            state.engine = Some(engine);
            state.config = Some(config);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
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
