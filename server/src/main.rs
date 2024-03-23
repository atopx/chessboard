// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// 修改后的代码
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};
use tokio::time::{interval, Duration};
mod chess;
mod common;
mod engine;
pub mod listen;
pub mod logger;
pub mod yolo;
const MODEL_PATH: &str = "../libs/model.onnx";

pub async fn start_listen(
    app: &mut tauri::App,
    handle: State<'static, Mutex<GobalHandle>>,
    name: String,
) -> Result<(), ()> {
    let mut _handle = handle.lock().unwrap();
    _handle.listen_window = Arc::new(Mutex::new(listen::ListenWindow::new(name)));

    // 启动新的线程, 执行定时任务
    let intervals = _handle.interval.clone(); // 先克隆出间隔时间
                                              // let listen_window = _handle.listen_window.clone();
    let listen_window = Arc::clone(&_handle.listen_window);
    let model = Arc::clone(&_handle.model);
    let engine = Arc::clone(&_handle.engine);

    tokio::spawn(async move {
        let mut intervals = interval(intervals);
        let mut listen_window = listen_window.lock().unwrap();
        let window = listen_window.as_mut().unwrap();
        let pic = window.capture();
        let origin_width = pic.width();
        let origin_height = pic.height();
        let boxes = model.predict(pic).unwrap();
        let (x, y, w, h) = common::detections_bound(origin_width, origin_height, &boxes).unwrap();
        window.set(x, y, w, h);
        drop(window);
        loop {
            intervals.tick().await;

            // let listen_window = listen_window.unwrap();
            if listen_window.is_none() {
                break;
            }
        }
    });

    Ok(())
}

pub fn stop_listen(app: &mut tauri::App, handle: State<'static, Mutex<GobalHandle>>) {
    // 取消原来的
    let mut handle = handle.lock().unwrap();
    handle.listen_window = Arc::new(Mutex::new(None));
}

pub struct GobalHandle {
    // 监听的应用
    pub listen_window: Arc<Mutex<Option<listen::ListenWindow>>>,
    // 模型
    pub model: Arc<yolo::Model>,
    // 引擎
    pub engine: Arc<engine::Engine>,
    // 扫描间隔
    pub interval: Duration,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let model_path = app.path_resolver().resolve_resource(MODEL_PATH).unwrap();
            let model = yolo::Model::new(model_path).unwrap();
            let mut engine = engine::Engine::new(MODEL_PATH);
            app.manage(GobalHandle {
                model: Arc::new(model),
                interval: Duration::from_millis(1000),
                listen_window: Arc::new(Mutex::new(None)),
                engine: Arc::new(engine),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
