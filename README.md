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

我要做一个象棋游戏的外部插件，我们先定义一些名词或对象名：
1. 目标象棋游戏：chessgame
2. 要做的外部插件：chessplugin
3. 窗口截图模块：windowcaputure
4. 棋盘识别模块：boardmodel
5. 引擎模块：chessengine
要实现的核心功能为：
- 循环检测chessgame
  - 截图
  - 识别棋盘
  - 判断是不是我方行棋
    - 是：调用引擎辅助
    - 否：下一轮检测
请帮我设计一下核心的判断逻辑，如何保证可以正确判断是不是我方行棋，需要为定义那些变量，如何比对等等



我的stop_listen函数会在`let mut state = STATE.lock().unwrap()`这里阻塞，为什么，如何解决
```rust
use std::{
    sync::{Arc, MutexGuard},
    thread,
    time::Duration,
};

use tauri::{AppHandle, Manager};
use tracing::{debug, info, trace};
use xcap::image::{ImageBuffer, Rgba};

use crate::{chess, common, engine::QueryRecords, listen::ListenWindow, STATE};

// 初始化Tauri的command处理
#[tauri::command]
pub fn start_listen(app: AppHandle, name: String) {
    trace!("start_listen");
    let state = STATE.clone();
    let mut state_lock: std::sync::MutexGuard<'_, crate::SharedState> = state.lock().unwrap();
    let model = state_lock.model.as_ref().unwrap();

    if state_lock.listen_thread.is_none() {
        // 初始化监听窗口模块
        let mut window = ListenWindow::new(name).unwrap(); // 创建窗口实例
        let image = window.capture();
        let image_h = image.height();
        let image_w = image.width();
        let detections = model.predict(image).unwrap();
        let (x, y, w, h) = common::detections_bound(image_w, image_h, &detections).unwrap();
        window.set(x, y, w, h);
        info!("WINDOW {} {} {} {}", x, y, w, h);

        // 启动后台线程进行截图和处理
        let state_for_thread = Arc::clone(&state);
        state_lock.listen_thread = Some(thread::spawn(move || {
            trace!("into thread");
            let mut state_thread_lock = state_for_thread.lock().unwrap();
            // 域变量
            trace!("域变量");
            let mut last_board = [[' '; 9]; 10];
            let mut expect_board = [[' '; 9]; 10];
            let mut first_connect = true;
            loop {
                // 检查是否需要停止监听
                if state_thread_lock.listen_thread.is_none() {
                    break;
                }
                // 循环固定间隔时间
                thread::sleep(Duration::from_millis(100));

                // 截图
                let image = window.capture();
                // 识别结果转换为棋盘
                let (camp, mut board) = get_board(&state_thread_lock, image);
                trace!("{:?} {:?}", camp, board);
                // 修复棋盘
                chess::board_fix(&camp, &mut board);

                // 判断棋盘是否是初始棋盘
                if chess::startpos(board) {
                    first_connect = false;
                    // 判断谁先
                    if chess::Camp::Red.eq(&camp) {
                        // 我方先手 立即分析
                        debug!("startpos, 我方先手");
                        if last_board == board {
                            // 防止重复分析
                            debug!("startpos, 我方先手, 防止重复分析");
                            continue;
                        }

                        last_board = board;

                        // 调用引擎查询
                        let fen = chess::board_fen(&camp, board);
                        let engine = state_thread_lock.engine.as_mut().unwrap();
                        let result = engine.go(&fen, 10, 1000);
                        if result.is_none() {
                            continue;
                        }
                        expect_board = analyse(&app, result.unwrap(), board);
                    } else {
                        // 对方先手 跳过分析
                        debug!("对方先手, 跳过分析");
                        last_board = board;
                    }
                    continue;
                }

                // 判断棋盘是否未发生变化
                if board == last_board {
                    debug!("棋盘未发生变化, 跳过分析");
                    continue;
                }

                // 判断棋盘是否为预期棋盘
                if board == expect_board {
                    // 跳过分析
                    debug!("棋盘为预期棋盘, 跳过分析");
                    last_board = expect_board;
                    continue;
                }

                // 棋盘可能在动画中, 延迟后重新确认
                thread::sleep(Duration::from_micros(100));
                let conf_image = window.capture();
                let (_, conf_board) = get_board(&state_thread_lock, conf_image);
                if conf_board != board {
                    // 如果不一致, 返回去重新识别
                    debug!("棋盘延迟确认失败");
                    continue;
                }

                // 检测棋盘是否有效
                if !chess::board_check(board) {
                    debug!("棋盘识别无效");
                    continue;
                }

                // 是否首次运行
                if first_connect {
                    // 立即分析, 调用引擎查询
                    debug!("首次启动, 立即分析");
                    let fen = chess::board_fen(&camp, board);
                    let engine = state_thread_lock.engine.as_mut().unwrap();
                    let result = engine.go(&fen, 10, 1000);
                    if result.is_none() {
                        continue;
                    }
                    expect_board = analyse(&app, result.unwrap(), board);
                    last_board = board;
                    first_connect = false;
                    continue;
                }

                // 非首次运行且一定发生变化了
                let (changed, board_state) = chess::board_diff(last_board, board);

                // 状态判断
                match board_state {
                    chess::BoardState::OneChanged => {
                        // 理论上不应该出现, 但有可能是动画问题影响, 直接continue
                        debug!("BoardState is OneChanged");
                        continue;
                    }
                    chess::BoardState::MoveChanged => {
                        // 合法移动, 这种应该是最正常, 判断是谁移动
                        if camp.eq(&changed.camp) {
                            // 我方移动
                            debug!("我方移动, {} -> {}, 跳过分析", changed.from, changed.to);
                            last_board = board;
                            continue;
                        } else {
                            // 对方移动, 跳过分析
                            debug!("对方移动, {} -> {}, 需要分析", changed.from, changed.to);
                            last_board = board;
                        }
                    }
                    chess::BoardState::UnknownChanged => {
                        // 理论上只有开始新的一局才会出现, 需要确认一次
                        debug!("棋局变化未知, 重新识别确认");
                        thread::sleep(Duration::from_micros(100));
                        let conf_image = window.capture();
                        let (_, conf_board) = get_board(&state_thread_lock, conf_image);
                        if conf_board != board {
                            // 如果不一致, 返回去重新识别
                            debug!("棋局变化未知, 重新识别不一致");
                            continue;
                        }
                        last_board = board;
                    }
                }

                // 引擎分析
                debug!("final 引擎分析");
                let fen = chess::board_fen(&camp, board);
                let engine = state_thread_lock.engine.as_mut().unwrap();
                let result = engine.go(&fen, 10, 1000);
                if result.is_none() {
                    continue;
                }
                expect_board = analyse(&app, result.unwrap(), board);
                continue;
            }
        }));
    }
}

#[tauri::command]
pub fn stop_listen() {
    info!("stop listen");
    let mut state = STATE.lock().unwrap();
    debug!("get state locked");
    if let Some(listen_thread) = state.listen_thread.take() {
        // 释放锁，停止后台线程
        debug!("释放锁，停止后台线程");
        drop(state);

        listen_thread.join().unwrap();
    }
    debug!("stoped");
}
```
