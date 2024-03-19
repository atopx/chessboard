use std::collections::HashMap;

use super::chess;
use super::yolo::detection::Detection;
use tauri::async_runtime::block_on;
use tokio::io::empty;
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

// 帮我实现这个对比函数, 返回值是发生变化的索引
pub fn diff_board(old_board: [[char; 9]; 10], new_board: [[char; 9]; 10]) -> Vec<(usize, usize)> {
    let mut diff_indices = Vec::new();

    for i in 0..10 {
        for j in 0..9 {
            if old_board[i][j] != new_board[i][j] {
                diff_indices.push((i, j));
            }
        }
    }

    diff_indices
}

pub fn black_board_fen(board: [[char; 9]; 10]) -> String {
    let mut fen = String::new();

    for y in (0..10).rev() {
        let mut empty = 0;
        for x in (0..9).rev() {
            let piece = board[y][x];
            if piece == ' ' {
                empty += 1;
            } else {
                if empty > 0 {
                    fen.push_str(&empty.to_string());
                    empty = 0;
                }
                fen.push(piece);
            }
        }
        if empty > 0 {
            fen.push_str(&empty.to_string());
        }
        fen.push('/');
    }
    fen.pop();
    fen
}

pub fn red_board_fen(board: [[char; 9]; 10]) -> String {
    let mut fen = String::new();
    for row in &board {
        let mut empty = 0;
        for &piece in row {
            if piece == ' ' {
                empty += 1;
            } else {
                if empty > 0 {
                    fen.push_str(&empty.to_string());
                    empty = 0;
                }
                fen.push(piece);
            }
        }
        if empty > 0 {
            fen.push_str(&empty.to_string());
        }
        fen.push('/');
    }
    fen.pop();
    fen
}

// board_to_map 棋盘数组转换为坐标模式
pub fn board_to_map(camp: chess::Camp, board: [[char; 9]; 10]) -> HashMap<&'static str, char> {
    let base_board = if camp == chess::Camp::Red {
        chess::RED_BOARD
    } else {
        chess::BLACK_BOARD
    };

    let mut positions = HashMap::new();

    for row in 0..10 {
        for col in 0..9 {
            let v = board[row][col];
            if v != ' ' {
                positions.insert(base_board[row][col], v);
            }
        }
    }
    positions
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_board_to_fen() {
        let board = [
            ['r', 'n', 'b', 'a', 'k', 'a', 'b', 'n', 'r'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'c', ' ', ' ', ' ', ' ', ' ', 'c', ' '],
            ['p', ' ', 'p', ' ', 'p', ' ', 'p', ' ', 'p'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['P', ' ', 'P', ' ', 'P', ' ', 'P', ' ', 'P'],
            [' ', 'C', ' ', ' ', 'C', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['R', 'N', 'B', 'A', 'K', 'A', 'B', 'N', 'R'],
        ];
        let expected_fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RNBAKABNR";
        assert_eq!(red_board_fen(board), expected_fen);
    }

    #[test]
    fn test_black_board_to_fen() {
        let board = [
            ['R', 'N', 'B', 'A', 'K', 'A', 'B', 'N', 'R'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', 'C', ' ', ' ', 'C', ' '],
            ['P', ' ', 'P', ' ', 'P', ' ', 'P', ' ', 'P'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['p', ' ', 'p', ' ', 'p', ' ', 'p', ' ', 'p'],
            [' ', 'c', ' ', ' ', ' ', ' ', ' ', 'c', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['r', 'n', 'b', 'a', 'k', 'a', 'b', 'n', 'r'],
        ];
        let expected_fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RNBAKABNR";
        assert_eq!(black_board_fen(board), expected_fen);
    }
}
