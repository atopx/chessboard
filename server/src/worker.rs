use std::{borrow::BorrowMut, sync::Arc, thread, time::Duration};

use crate::{
    chess::{self, BoardState, Camp},
    common,
    listen::ListenWindow,
    STATE,
};

// 初始化Tauri的command处理
#[tauri::command]
pub fn start_listen(name: String) {
    let state = STATE.clone();
    let mut state_lock = state.lock().unwrap();
    let model = state_lock.model.as_ref().unwrap();
    let engine = state_lock.engine.as_ref().unwrap();

    if state_lock.listen_thread.is_none() {
        // 初始化监听窗口模块
        let mut window = ListenWindow::new(name).unwrap(); // 创建窗口实例
        let image = window.capture();
        let image_h = image.height();
        let image_w = image.width();
        let detections = model.predict(image).unwrap();
        let (x, y, w, h) = common::detections_bound(image_w, image_h, &detections).unwrap();
        window.set(x, y, w, h);

        // 启动后台线程进行截图和处理
        let state_for_thread = Arc::clone(&state);
        // let model = state_lock.model.as_ref().unwrap();
        state_lock.listen_thread = Some(thread::spawn(move || {
            let mut state_thread_lock = state_for_thread.lock().unwrap();
            // 共享实例
            let model = state_thread_lock.model.as_ref().unwrap();
            let mut engine_state = state_for_thread.lock().unwrap();
            let engine = engine_state.engine.as_mut().unwrap();

            // 域变量
            let mut current_board = [[' '; 9]; 10];
            let mut last_board = [[' '; 9]; 10];
            let mut expect_board = [[' '; 9]; 10];
            let mut current_camp = chess::Camp::None;
            let mut first_connect = true;
            let mut board_state = chess::BoardState::NotChanged;

            loop {
                // 检查是否需要停止监听
                if state_thread_lock.listen_thread.is_none() {
                    break;
                }
                // 截图
                let image = window.capture();
                // 识别
                let detections = model.predict(image).unwrap();
                // 识别结果转换为棋盘
                let (camp, mut board) = common::detections_to_board(detections).unwrap();
                // 修复棋盘
                chess::board_fix(&camp, &mut board);
                // 判断棋盘是否是初始棋盘
                if chess::startpos(board) {
                    // 判断谁先
                    if Camp::Red.eq(&camp) {
                        // 我方先手 立即分析
                        current_camp = Camp::Red;

                        // 转换为FEN
                        let mut fen = chess::board_fen(board);
                        fen.push(' ');
                        fen.push(camp.to_char());

                        // 调用引擎查询
                        let result = engine.go(&fen, 10, 1000).unwrap();

                        // TODO 发送结果到前端
                    } else {
                        // TODO 对方先手 跳过分析
                        current_camp = Camp::Black;
                    }
                    continue;
                }

                // 检测棋盘是否有效
                if !chess::board_check(board) {
                    thread::sleep(Duration::from_millis(50));
                    continue;
                }

                // 是否首次运行
                if first_connect {
                    // TODO 立即分析

                    continue;
                }

                // 非首次运行状态判断

                // 状态判断

                // 等待一段时间，避免过度占用CPU
                thread::sleep(Duration::from_millis(100));
            }
        }));
    }
}
