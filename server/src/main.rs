// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use engine::Engine;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use tauri::{async_runtime, Manager, State};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{interval, Duration};
mod chess;
mod common;
mod engine;
pub mod yolo;

const MODEL_PATH: &str = "../libs/model.onnx";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(app: tauri::AppHandle, name: &str) -> String {
    let model_path = app.path_resolver().resolve_resource(MODEL_PATH).unwrap();
    let model = yolo::Model::new(model_path).unwrap();
    let window = common::get_windows(name).unwrap();
    let image = window.capture_image().unwrap();
    let detections = model.predict(image).unwrap();
    let count = detections.len();
    println!("detections count {}", count);
    format!("detections count {}", count)
}

pub fn start_listen(app: &mut tauri::App, handle: &mut State<&mut GobalHandle>, name: &str) {
    if let Some(cancel_rx) = &handle.cancel_rx {
        // 取消原来的
        cancel_rx.send(());
        handle.cancel_rx = None;
    }
    let window = common::get_windows(name).unwrap();
    let (tx, mut rx) = mpsc::channel::<()>(1);
    // 启动新的线程
    let config = handle.config.clone();
    tokio::spawn(async move {
        let mut interval = interval(config.interval);
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // 执行你的定时任务
                    println!("Tick");
                }
                _ = rx.recv() => {
                    // 接收到取消信号，退出循环
                    break;
                }
            }
        }
    });
    handle.cancel_rx = Some(tx);
}

pub fn stop_listen(app: &mut tauri::App, handle: &mut State<GobalHandle>) {
    if let Some(cancel_rx) = &handle.cancel_rx {
        // 取消原来的
        cancel_rx.send(());
    }
}

/// 启动识别的Timer循环，并返回一个用于取消该Timer的发送者。
pub async fn start_timer(interval_duration: Duration) -> mpsc::Sender<()> {
    let (tx, mut rx) = mpsc::channel::<()>(1);

    // 使用tokio::spawn来在后台运行Timer循环。
    tokio::spawn(async move {
        let mut interval = interval(interval_duration);

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // 执行你的定时任务
                    println!("Tick");
                }
                _ = rx.recv() => {
                    // 接收到取消信号，退出循环
                    break;
                }
            }
        }
    });

    // 返回用于取消Timer的发送者。
    tx
}

struct DetectTimer {
    window: xcap::Window,
    interval: Duration,
    engine: Engine,
    cancel: Receiver<()>,
}

#[derive(Clone, Copy)]
struct Config {
    interval: Duration,
}

struct GobalHandle {
    // window: Mutex<Option<xcap::Window>>,
    model: yolo::Model,
    cancel_rx: Option<mpsc::Sender<()>>,
    config: Config,
}

#[tokio::main]
async fn main() {
    // 配置tokio运行环境
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .setup(|app| {
            let (tx, rx): (Sender<String>, Receiver<String>) = channel();
            // let mut chess_board = ChessBoard::new();
            let model_path = app.path_resolver().resolve_resource(MODEL_PATH).unwrap();
            let model = yolo::Model::new(model_path).unwrap();
            // let window = common::get_windows(name).unwrap();
            app.manage(GobalHandle {
                model: model,
                cancel_rx: todo!(),
                config: Config {
                    interval: Duration::from_millis(1000),
                },
            });

            thread::spawn(move || {
                loop {
                    {
                        // let image = window.capture_image().unwrap();
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
