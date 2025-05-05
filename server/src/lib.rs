use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::RwLock;
use std::thread;

use engine::Engine;
use tauri::path::BaseDirectory;
use tauri::AppHandle;
use tauri::Manager;

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
    config: Arc<RwLock<config::Config>>,
    engine: Arc<Mutex<Engine>>,
    listen_thread: Mutex<Option<thread::JoinHandle<()>>>,
}

static SHARED_STATE: OnceLock<SharedState> = OnceLock::new();

#[tauri::command]
fn reload_engine(app: AppHandle) {
    let lib_path = app
        .path()
        .resolve("../libs/pikafish", BaseDirectory::Resource)
        .unwrap();
    let state = SHARED_STATE.get().unwrap();
    let engine_config = state.config.read().unwrap().engine;
    state
        .engine
        .lock()
        .unwrap()
        .reload(&lib_path, &engine_config);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            logger::init_tracer(tracing::Level::DEBUG, &app.path().app_data_dir().unwrap());

            let _ = SHARED_STATE.get_or_init(|| {
                let config = config::Config::load(&app.path().config_dir().unwrap());
                let lib_path = app
                    .path()
                    .resolve("../libs/pikafish", BaseDirectory::Resource)
                    .unwrap();
                let mut engine = engine::Engine::new(&lib_path);
                engine.set_show_wdl(config.engine.show_wdl);
                engine.set_hash(config.engine.hash);
                engine.set_threads(config.engine.threads);

                SharedState {
                    config: Arc::new(RwLock::new(config)),
                    engine: Arc::new(Mutex::new(engine)),
                    listen_thread: Mutex::new(None),
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            reload_engine,
            listen::list_windows,
            worker::start_listen,
            worker::stop_listen,
            config::get_engine_config,
            config::set_engine_depth,
            config::set_engine_time,
            config::set_engine_threads,
            config::set_engine_hash,
            config::set_chessdb,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
