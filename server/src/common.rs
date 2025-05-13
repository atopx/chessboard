use tracing::trace;

use crate::chess;
use crate::yolo;

// detections_bound 获取截图的边界
pub fn detections_bound(
    origin_width: u32, origin_height: u32, detections: &[yolo::Detection],
) -> Result<(u32, u32, u32, u32), String> {
    // 找到棋盘（label == '0'）
    let board_det = detections.iter().find(|d| d.label == '0').ok_or("未识别到棋盘")?;

    // 计算模型图到原图的缩放
    let scale_x = origin_width as f32 / yolo::IMAGE_WIDTH as f32;
    let scale_y = origin_height as f32 / yolo::IMAGE_HEIGHT as f32;

    // 模型坐标 → 原图坐标
    let bx0 = (board_det.x0 * scale_x).max(0.0);
    let by0 = (board_det.y0 * scale_y).max(0.0);
    let bx1 = (board_det.x1 * scale_x).min(origin_width as f32);
    let by1 = (board_det.y1 * scale_y).min(origin_height as f32);

    // 计算原图下的“半格”尺寸
    let board_w = bx1 - bx0;
    let board_h = by1 - by0;
    let half_cell_x = board_w / 9.0 / 2.0;
    let half_cell_y = board_h / 10.0 / 2.0;

    // 计算裁剪框左上
    let crop_x = (bx0 - half_cell_x).max(0.0) as u32;
    let crop_y = (by0 - half_cell_y).max(0.0) as u32;

    // 计算裁剪框右下，在原图范围内
    let x1p = (bx1 + half_cell_x).min(origin_width as f32);
    let y1p = (by1 + half_cell_y).min(origin_height as f32);

    // 宽高 = 右下 - 左上
    let width = (x1p - crop_x as f32) as u32;
    let height = (y1p - crop_y as f32) as u32;

    Ok((crop_x, crop_y, width, height))
}

const MODEL_CELL_W: f32 = yolo::IMAGE_WIDTH as f32 / 9.0;
const MODEL_CELL_H: f32 = yolo::IMAGE_HEIGHT as f32 / 10.0;

// detections_to_board 识别结果转换为棋盘结构
pub fn detections_to_board(detections: &[yolo::Detection]) -> Result<(chess::Camp, [[char; 9]; 10]), String> {
    let mut camp = chess::Camp::None;
    let mut board = [[' '; 9]; 10];

    match detections.iter().find(|&&x| x.label == '0') {
        Some(_) => {
            for det in detections.iter().filter(|d| d.label != '0') {
                // 中心点
                let cx = (det.x0 + det.x1) / 2.0;
                let cy = (det.y0 + det.y1) / 2.0;
                // 行列：x 轴分成 9 格，y 轴分成 10 格
                let col = (cx / MODEL_CELL_W).floor() as usize; // 0–8
                let row = (cy / MODEL_CELL_H).floor() as usize; // 0–9
                trace!("{} row={} col={}", det.label, row, col);

                // 边界处理
                if !(0..=8).contains(&col) || !(0..=9).contains(&row) {
                    continue;
                }

                // 构建board
                board[row][col] = det.label;

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
