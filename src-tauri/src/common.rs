use tracing::trace;

use super::chess;
use super::yolo::Detection;

// detections_bound 获取截图的边界
pub fn detections_bound(
    w: u32, h: u32, detections: &[Detection],
) -> Result<(u32, u32, u32, u32), String> {
    match detections.iter().find(|&&x| x.label == '0') {
        Some(board_det) => {
            let rate_x = w as f32 / 640.0;
            let rate_y = h as f32 / 640.0;
            let space_x = board_det.w / 8.0;
            let space_y = board_det.h / 9.0;
            let x = ((board_det.x0 - space_x) * rate_x) as u32;
            let y = ((board_det.y0 - space_y) * rate_y) as u32;
            let w = ((board_det.w + space_x * 2.0) * rate_x) as u32;
            let h = ((board_det.h + space_y * 2.0) * rate_y) as u32;
            Ok((x, y, w, h))
        }
        None => Err(String::from("1234")),
    }
}

// detections_to_board 识别结果转换为棋盘结构
pub fn detections_to_board(detections: Vec<Detection>) -> Result<(chess::Camp, [[char; 9]; 10]), String> {
    let mut camp = chess::Camp::None;
    let mut board = [[' '; 9]; 10];

    match detections.iter().find(|&&x| x.label == '0') {
        Some(board_det) => {
            let space_x = board_det.w / 8.0;
            let space_y = board_det.h / 9.0;
            for det in detections.iter() {
                if det.label == board_det.label {
                    continue;
                }
                // 计算棋子定位(转整数: +0.5向下取整)
                let col = ((det.x - board_det.x0) / space_x + 0.5) as i32;
                let row = ((det.y - board_det.y0) / space_y + 0.5) as i32;
                trace!("{} row={} col={}", det.label, row, col);

                // 边界处理
                if !(0..=8).contains(&col) || !(0..=9).contains(&row) {
                    continue;
                }

                // 构建board
                board[row as usize][col as usize] = det.label;

                // 判断阵营
                if camp == chess::Camp::None && (3..=5).contains(&col) && row >= 7 {
                    match det.label {
                        'k' => camp = chess::Camp::Black,
                        'K' => camp = chess::Camp::Red,
                        _ => {}
                    }
                }
            }
        }
        None => return Err("not board".to_string()),
    }
    Ok((camp, board))
}
