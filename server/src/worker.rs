use std::{
    sync::{Arc, MutexGuard},
    thread,
    time::Duration,
};

use tauri::{AppHandle, Manager};
use tracing::{debug, info, trace};
use xcap::image::{ImageBuffer, Rgba};

use crate::{chess, common, engine::QueryRecords, listen::ListenWindow, STATE};

pub fn get_board(
    state: &MutexGuard<'_, crate::SharedState>,
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Option<(chess::Camp, [[char; 9]; 10])> {
    let model = state.model.as_ref().unwrap();
    let data = model.predict(image).unwrap();
    if let Ok((camp, mut board)) = common::detections_to_board(data) {
        chess::board_fix(&camp, &mut board);
        return Some((camp, board));
    }
    None
}

pub fn analyse(
    app: &AppHandle,
    mut result: QueryRecords,
    board: [[char; 9]; 10],
) -> [[char; 9]; 10] {
    // 引擎结果翻译为中文
    // let mut tmp_board = board;
    let best_pv = result.pvs.first().unwrap();
    let best_move = chess::board_move_chinese(board, best_pv);
    let expect_board = chess::board_move(board, best_pv);
    let mut tmp_board = expect_board;
    result.moves.push(best_move);
    for pv in result.pvs.iter().skip(1) {
        let mv = chess::board_move_chinese(tmp_board, pv);
        result.moves.push(mv);
        tmp_board = chess::board_move(tmp_board, pv);
    }
    // 把结果发送给前端
    info!("分析结果 {:?}", result);
    app.emit_all("event", result).unwrap();

    // 返回一个预期board
    expect_board
}

// 初始化Tauri的command处理
#[tauri::command]
pub fn start_listen(app: AppHandle, name: String) {
    trace!("start_listen");
    let state = STATE.clone();

    if state.lock().unwrap().listen_thread.is_none() {
        // 初始化监听窗口模块
        let mut window = ListenWindow::new(name).unwrap(); // 创建窗口实例
        let image = window.capture();
        let image_h = image.height();
        let image_w = image.width();

        let detections = state
            .lock()
            .unwrap()
            .model
            .as_ref()
            .unwrap()
            .predict(image)
            .unwrap();

        let (x, y, w, h) = common::detections_bound(image_w, image_h, &detections).unwrap();
        window.set(x, y, w, h);
        info!("WINDOW {} {} {} {}", x, y, w, h);

        // 启动后台线程进行截图和处理
        let state_for_thread = Arc::clone(&state);
        state.lock().unwrap().listen_thread = Some(thread::spawn(move || {
            trace!("into thread");
            // let mut state_thread_lock = state_for_thread.lock().unwrap();
            // 域变量
            trace!("域变量");
            let mut last_board = [[' '; 9]; 10];
            let mut expect_board = [[' '; 9]; 10];
            let mut first_connect = true;
            loop {
                // 循环固定间隔时间
                thread::sleep(Duration::from_millis(200));

                // 检查是否需要停止监听
                if state_for_thread.lock().unwrap().listen_thread.is_none() {
                    debug!("listen stoped");
                    break;
                }

                // 截图
                let image = window.capture();
                // 识别结果转换为棋盘

                let r = get_board(&state_for_thread.lock().unwrap(), image);
                if r.is_none() {
                    continue;
                }
                let (camp, mut board) = r.unwrap();
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
                        let mut state_lock = state_for_thread.lock().unwrap();
                        let engine = state_lock.engine.as_mut().unwrap();
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
                let r = get_board(&state_for_thread.lock().unwrap(), conf_image);
                if r.is_none() {
                    continue;
                }
                let (_, conf_board) = r.unwrap();

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
                    let mut state_lock = state_for_thread.lock().unwrap();
                    let engine = state_lock.engine.as_mut().unwrap();
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
                        let r = get_board(&state_for_thread.lock().unwrap(), conf_image);
                        if r.is_none() {
                            continue;
                        }
                        let (_, conf_board) = r.unwrap();
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
                let mut state_lock = state_for_thread.lock().unwrap();
                let engine = state_lock.engine.as_mut().unwrap();
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
