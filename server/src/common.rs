use super::chess;
use super::yolo::detection::Detection;
use xcap::Window;

pub enum ChessState {
    // 空闲
    None,
    // 识别棋盘
    Detect,
    // 引擎分析
    Analyse,
}

pub fn get_windows(title: &str) -> ort::Result<xcap::Window, &str> {
    let windows = Window::all().unwrap();
    for window in windows {
        if window.is_minimized() {
            continue;
        }
        if window.title() != title {
            continue;
        }
        return Ok(window);
    }
    Err("未找到窗口")
}

pub struct Position {
    pub key: String,
    pub value: char,
}

impl Position {
    pub fn new(key: &str, value: char) -> Self {
        Self {
            key: key.to_string(),
            value,
        }
    }
}

// detections_to_board 识别结果转换为棋盘结构
pub fn detections_to_board(
    detections: Vec<Detection>,
) -> Result<(chess::Camp, [[char; 9]; 10]), &'static str> {
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
                println!("{} row={} col={}", det.label, row, col);

                // 边界处理
                if col < 0 || col > 8 || row < 0 || row > 9 {
                    continue;
                }

                // 构建board
                board[row as usize][col as usize] = det.label;

                // 判断阵营
                if camp != chess::Camp::None && 3 <= col && col <= 5 && row >= 7 {
                    match det.label {
                        'k' => camp = chess::Camp::Black,
                        'K' => camp = chess::Camp::Red,
                        _ => {}
                    }
                }
            }
        }
        None => return Err("not board"),
    }
    Ok((camp, board))
}
