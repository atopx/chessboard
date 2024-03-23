# 中国象棋学习工具

整体采用轻量级跨平台桌面应用方案`tauri`框架

前端: Vue3 + NaiveUI + TypeScript
后端: Rust + c + yolo + onnxruntime

# 线程

### 主线程

### 线程1

重复判断棋盘是否发生变化:

- 是 => 发送move事件到前端
  - 是否需要分析:
    - 是 => 发送fen到分析队列

### 线程2

监听分析队列，收到事件后:
调用云库查询, 是否查询成功 - 是 => 是否满足阈值设置: - 是 => 发送日志到前端 - 否 => 调用引擎 => 发送日志到前端 - 否 => 调用引擎 => 发送日志到前端

为这个函数编写一个单元测试(注意这里的`async runtime`用的是tokio)

中国象棋FEN的格式参考国际象棋基本原理，但有一些变化。

- 中国象棋棋盘大小固定为10行9列
- 中国象棋棋子使用单个字符表示，例如：k表示将，a表示士，b表示象，n表示马，r表示车，c表示炮，p表示兵。大写字符表示红方棋子，小写字符表示黑方棋子。
- 中国象棋棋盘默认黑方在上，红方在下方，FEN的的记录从上到下、从左排列
- 一个初始的棋盘FEN为：`rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w`, 其中最后的w代表此时该红方行棋。

现在请实现`pub async fn board_to_fen(camp: Camp, board: [[char; 9]; 10]) -> String {}`

帮我写实现parse函数, text的示例如下: score:0,depth:38,pv:h9g7|b0a2|h7i7|h0g2|i9h9|g3g4|c6c5|b2c2|b9c7|a0b0|c7d5|b0b4|b7d7|i0h0|h9h0|g2h0|d5e3|h0g2|d7e7|g2e3|e7e3|f0e1|c9e7|b4e4|e3a3|e2e6|d9e8|e6e5|a9d9|e4a4|a3i3|a4a6|i7h7|c2h2|d9d5|a6a9|d5d9|a9a6

```rust
// 引入必要的库和模块
use tauri::{Builder, Context, Runnable, generate_context, AppHandle, Manager};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

// 你的模块
mod engine;
mod model;
mod window;

use engine::Engine;
use model::Model;
use window::ListenWindow;

// 全局共享状态，用Arc和Mutex包装以实现线程安全共享
struct SharedState {
    engine: Engine,
    model: Model,
    listen_window: Option<ListenWindow>,
    listen_thread: Option<thread::JoinHandle<()>>,
}

lazy_static! {
    // 使用lazy_static来创建一个全局的、可变的、线程安全的单例
    static ref STATE: Arc<Mutex<SharedState>> = Arc::new(Mutex::new(SharedState {
        engine: Engine::new(), // 假设有这样的构造函数
        model: Model::new(),   // 同上
        listen_window: None,
        listen_thread: None,
    }));
}

// Tauri应用的主入口
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_listen,
            stop_listen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 初始化Tauri的command处理
#[tauri::command]
fn start_listen(ctx: Context) {
    let state = STATE.clone();
    let mut state_lock = state.lock().unwrap();

    if state_lock.listen_thread.is_none() {
        let window = ListenWindow::new(); // 创建窗口实例
        state_lock.listen_window = Some(window);

        // 启动后台线程进行截图和处理
        let state_for_thread = Arc::clone(&state);
        state_lock.listen_thread = Some(thread::spawn(move || {
            let mut state_thread_lock = state_for_thread.lock().unwrap();
            loop {
                // TODO: 实现截图和逻辑处理
                // 注意这里要处理好线程同步和资源共享的问题

                // 检查是否需要停止监听
                if state_thread_lock.listen_thread.is_none() {
                    break;
                }

                // 等待一段时间，避免过度占用CPU
                thread::sleep(Duration::from_millis(100));
            }
        }));
    }

    // 返回响应给前端，例如确认开始监听的消息
    ctx.emit("listen_started", ()).unwrap();
}

#[tauri::command]
fn stop_listen(ctx: Context) {
    let mut state = STATE.lock().unwrap();

    if let Some(listen_thread) = state.listen_thread.take() {
        // 停止后台线程
        drop(state); // 释放锁，以便线程可以退出循环
        listen_thread.join().unwrap();
    }

    // 关闭窗口、清理资源等操作可以在这里进行

    // 返回响应给前端，例如确认停止监听的消息
    ctx.emit("listen_stopped", ()).unwrap();
}

// 这里你可能需要为Engine、Model和ListenWindow实现适当的构造函数、方法和逻辑。
// ...
```
