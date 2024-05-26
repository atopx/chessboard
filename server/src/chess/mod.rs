use std::{cmp::Ordering, collections::HashMap};

use serde::Serialize;
use tracing::warn;

pub const BOARD_MAP: [[&str; 9]; 10] = [
    ["a9", "b9", "c9", "d9", "e9", "f9", "g9", "h9", "i9"],
    ["a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", "i8"],
    ["a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "i7"],
    ["a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "i6"],
    ["a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "i5"],
    ["a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "i4"],
    ["a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "i3"],
    ["a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "i2"],
    ["a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "i1"],
    ["a0", "b0", "c0", "d0", "e0", "f0", "g0", "h0", "i0"],
];

#[derive(Debug, Serialize)]
pub struct Position {
    piece: char,
    pos: String,
}

#[derive(Debug)]
pub enum BoardChangeState {
    // 变化了一个棋子
    One,
    // 正常一步棋移动
    Move,
    // 未知多个变化
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Serialize)]
pub enum Camp {
    #[default]
    None,
    Red,
    Black,
}

impl Camp {
    pub fn to_char(&self) -> char {
        match self {
            Camp::None => '0',
            Camp::Red => 'w',
            Camp::Black => 'b',
        }
    }

    pub fn from_piece(p: char) -> Self {
        if p > 'Z' {
            Self::Black
        } else {
            Self::Red
        }
    }

    pub fn is_black(&self) -> bool {
        Camp::Black.eq(self)
    }
}

const BLACK_VERTICALS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
const RED_VERTICALS: [char; 9] = ['九', '八', '七', '六', '五', '四', '三', '二', '一'];

const RED_STARTPOS: [[char; 9]; 10] = [
    ['r', 'n', 'b', 'a', 'k', 'a', 'b', 'n', 'r'],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', 'c', ' ', ' ', ' ', ' ', ' ', 'c', ' '],
    ['p', ' ', 'p', ' ', 'p', ' ', 'p', ' ', 'p'],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['P', ' ', 'P', ' ', 'P', ' ', 'P', ' ', 'P'],
    [' ', 'C', ' ', ' ', ' ', ' ', ' ', 'C', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['R', 'N', 'B', 'A', 'K', 'A', 'B', 'N', 'R'],
];

pub fn get_verticals(p: char) -> [char; 9] {
    if p > 'Z' {
        BLACK_VERTICALS
    } else {
        RED_VERTICALS
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Changed {
    pub piece: char,
    pub camp: Camp,
    pub from: String,
    pub to: String,
}

impl Changed {
    pub fn from_pv(pv: &str, board: [[char; 9]; 10]) -> Self {
        let (from, to) = pv.split_at(2);
        let mut cs = pv.chars();
        let from_x = cs.next().unwrap() as usize - 97;
        let from_y = 57 - cs.next().unwrap() as usize;
        let piece = board[from_y][from_x];
        Self { piece, camp: Camp::from_piece(piece), from: from.to_string(), to: to.to_string() }
    }
}

// 对比棋盘, 返回值是发生变化的索引
pub fn board_diff(old_board: [[char; 9]; 10], board: [[char; 9]; 10]) -> (Changed, BoardChangeState) {
    let mut changed = Changed::default();
    let mut count = 0;
    for y in 0..10 {
        for x in 0..9 {
            if old_board[y][x] != board[y][x] {
                count += 1;
                match board[y][x] {
                    ' ' => {
                        changed.piece = old_board[y][x];
                        changed.from = BOARD_MAP[y][x].to_string();
                        changed.camp = Camp::from_piece(changed.piece);
                    }
                    _ => changed.to = BOARD_MAP[y][x].to_string(),
                }
            }
        }
    }

    match count {
        1 => (changed, BoardChangeState::One),
        2 => {
            if changed.from.is_empty() || changed.to.is_empty() {
                (changed, BoardChangeState::One)
            } else {
                (changed, BoardChangeState::Move)
            }
        }
        _ => (changed, BoardChangeState::Unknown),
    }
}

pub struct Move {
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
}

impl Move {
    pub fn new(iccs: &str) -> Self {
        let mut cs = iccs.chars();
        let from_x = cs.next().unwrap() as usize - 97;
        let from_y = 57 - cs.next().unwrap() as usize;
        let to_x = cs.next().unwrap() as usize - 97;
        let to_y = 57 - cs.next().unwrap() as usize;
        Self { from_x, from_y, to_x, to_y }
    }
}

pub fn board_move(board: [[char; 9]; 10], iccs: &str) -> [[char; 9]; 10] {
    let mv = Move::new(iccs);
    let mut new_board = board;
    let p = new_board[mv.from_y][mv.from_x];
    new_board[mv.to_y][mv.to_x] = p;
    new_board[mv.from_y][mv.from_x] = ' ';
    new_board
}

// 棋盘转换FEN逻辑
pub fn board_fen(camp: &Camp, board: [[char; 9]; 10]) -> String {
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
    fen.push(' ');
    fen.push(camp.to_char());
    fen
}

// 检测棋盘是否合法
pub fn board_check(board: [[char; 9]; 10]) -> bool {
    let mut bk = 0;
    let mut ba = 0;
    let mut bb = 0;
    let mut bc = 0;
    let mut bp = 0;
    let mut br = 0;
    let mut bn = 0;
    let mut rk = 0;
    let mut ra = 0;
    let mut rb = 0;
    let mut rc = 0;
    let mut rp = 0;
    let mut rr = 0;
    let mut rn = 0;

    for (y, row) in board.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            match col {
                'k' => {
                    bk += 1;
                    if y > 2 || !(3..=5).contains(&x) {
                        warn!("黑方'将'不在合法位置内");
                        return false;
                    }
                }
                'a' => {
                    ba += 1;
                    if !(x == 3 && (y == 0 || y == 2))
                        && !(x == 4 && y == 1)
                        && !(x == 5 && (y == 0 || y == 2))
                    {
                        warn!("黑方'士'不在合法位置内, ({}行{}列)", y, x);
                        return false;
                    }
                }
                'b' => {
                    bb += 1;
                    if !(y == 0 && (x == 2 || x == 6))
                        && !(y == 2 && (x == 0 || x == 4 || x == 8))
                        && !(y == 4 && (x == 2 || x == 6))
                    {
                        warn!("黑方'象'不在合法位置内, ({}行{}列)", y, x);
                        return false;
                    }
                }
                'c' => {
                    bc += 1;
                }
                'p' => {
                    bp += 1;
                    if y < 3 {
                        warn!("黑方'兵'不在合法位置内, ({}行{}列)", y, x);
                        return false;
                    }
                    if y < 5 && x % 2 == 1 {
                        warn!("黑方'兵'不在合法位置内, ({}行{}列)", y, x);
                        return false;
                    }
                }
                'r' => {
                    br += 1;
                }
                'n' => {
                    bn += 1;
                }
                'K' => {
                    rk += 1;
                    if y < 7 || !(3..=5).contains(&x) {
                        warn!("红方'将'不在合法位置内, ({}行{}列)", y, x);
                        return false;
                    }
                }
                'A' => {
                    ra += 1;
                    if !(x == 3 && (y == 7 || y == 9))
                        && !(x == 4 && y == 8)
                        && !(x == 5 && (y == 7 || y == 9))
                    {
                        warn!("红方'士'不在合法位置内, ({}行{}列)", y, x);
                        return false;
                    }
                }
                'B' => {
                    rb += 1;
                    if !(y == 9 && (x == 2 || x == 6))
                        && !(y == 7 && (x == 0 || x == 4 || x == 8))
                        && !(y == 5 && (x == 2 || x == 6))
                    {
                        warn!("红方'象'不在合法位置内, ({}行{}列)", y, x);
                        return false;
                    }
                }
                'C' => {
                    rc += 1;
                }
                'P' => {
                    rp += 1;
                    if y > 6 {
                        warn!("红方'兵'不在合法位置内, ({}行{}列)", y, x);
                        return false;
                    }
                    if y > 4 && x % 2 == 1 {
                        warn!("红方'兵'不在合法位置内, ({}行{}列)", y, x);
                        return false;
                    }
                }
                'R' => {
                    rr += 1;
                }
                'N' => {
                    rn += 1;
                }
                _ => {}
            }
        }
    }

    if bk != 1 || rk != 1 {
        warn!("黑方或红方'将'超出合法数量(红:{}, 黑:{})", rk, bk);
        return false;
    }
    if ba > 2 || ra > 2 {
        warn!("黑方或红方'士'超出合法数量(红:{}, 黑:{})", ra, ba);
        return false;
    }
    if bb > 2 || rb > 2 {
        warn!("黑方或红方'象'超出合法数量(红:{}, 黑:{})", rb, bb);
        return false;
    }
    if bc > 2 || rc > 2 {
        warn!("黑方或红方'炮'超出合法数量(红:{}, 黑:{})", rc, bc);
        return false;
    }
    if br > 2 || rr > 2 {
        warn!("黑方或红方'车'超出合法数量(红:{}, 黑:{})", rr, br);
        return false;
    }
    if bn > 2 || rn > 2 {
        warn!("黑方或红方'马'超出合法数量(红:{}, 黑:{})", rn, bn);
        return false;
    }
    if bp > 5 || rp > 5 {
        warn!("黑方或红方'兵'超出合法数量(红:{}, 黑:{})", rp, bp);
        return false;
    }
    true
}

pub const fn get_piece_name(piece: char) -> char {
    match piece {
        'K' => '帅',
        'k' => '将',
        'A' => '仕',
        'a' => '士',
        'B' => '相',
        'b' => '象',
        'N' => '马',
        'n' => '马',
        'R' => '车',
        'r' => '车',
        'C' => '炮',
        'c' => '炮',
        'P' => '兵',
        'p' => '卒',
        _ => ' ',
    }
}

pub fn startpos(board: [[char; 9]; 10]) -> bool {
    board == RED_STARTPOS
}

pub fn board_fix(camp: &Camp, board: &mut [[char; 9]; 10]) {
    if Camp::Black.eq(camp) {
        board.reverse();
        for i in board {
            i.reverse()
        }
    }
}

// board_to_map 棋盘数组转换为坐标模式
pub fn board_map(board: [[char; 9]; 10]) -> Vec<Position> {
    let mut position = vec![];

    for row in 0..10 {
        for col in 0..9 {
            position.push(Position { piece: board[row][col], pos: BOARD_MAP[row][col].to_string() });
        }
    }
    position
}

fn overlap_piece_y(board: [[char; 9]; 10], x: usize, y: usize, piece: char) -> Vec<usize> {
    let mut other_ys = Vec::new();
    for (i, value) in board.iter().enumerate() {
        if i != y && value[x] == piece {
            other_ys.push(i);
        }
    }
    other_ys
}

fn overlap_piece_xy(
    board: [[char; 9]; 10],
    from_x: usize,
    piece: char,
) -> Option<HashMap<usize, Vec<usize>>> {
    let mut other_xys = HashMap::new();
    for x in 0..9 {
        if x != from_x {
            let other_ys = overlap_piece_y(board, x, 10, piece); // 注意：这里将y设置为10是一个技巧，因为我们的棋盘只有10行，所以永远不会找到y==10的情况。但这样做并不是最好的方式。
            if other_ys.len() > 1 {
                other_xys.insert(x, other_ys);
                return Some(other_xys);
            }
        }
    }
    None
}

// 棋子坐标移动转中文模式
pub fn board_move_chinese(board: [[char; 9]; 10], iccs: &str) -> String {
    let mut chinese = String::new();
    let mv = Move::new(iccs);
    let piece = board[mv.from_y][mv.from_x];
    let verticals = get_verticals(piece);
    match piece {
        'K' => {
            chinese.push(get_piece_name(piece));
            chinese.push(verticals[mv.from_x]);

            match mv.from_y.cmp(&mv.to_y) {
                Ordering::Equal => {
                    // 平
                    chinese.push('平');
                    chinese.push(verticals[mv.to_x]);
                }
                Ordering::Less => {
                    // 进
                    chinese.push('退');
                    chinese.push(verticals[8]);
                }
                Ordering::Greater => {
                    // 退
                    chinese.push('进');
                    chinese.push(verticals[8]);
                }
            }
        }
        'k' => {
            chinese.push(get_piece_name(piece));
            chinese.push(verticals[mv.from_x]);

            match mv.from_y.cmp(&mv.to_y) {
                Ordering::Less => {
                    // 退
                    chinese.push('进');
                    chinese.push(verticals[0]);
                }
                Ordering::Greater => {
                    // 进
                    chinese.push('退');
                    chinese.push(verticals[0]);
                }
                Ordering::Equal => {
                    // 平
                    chinese.push('平');
                    chinese.push(verticals[mv.to_x]);
                }
            }
        }
        'A' | 'B' => {
            chinese.push(get_piece_name(piece));
            chinese.push(verticals[mv.from_x]);
            // 士、象只有进退
            match mv.from_y.cmp(&mv.to_y) {
                Ordering::Less => chinese.push('退'),
                _ => chinese.push('进'),
            }
            chinese.push(verticals[mv.to_x]);
        }
        'a' | 'b' => {
            chinese.push(get_piece_name(piece));
            chinese.push(verticals[mv.from_x]);
            // 士、象只有进退
            match mv.from_y.cmp(&mv.to_y) {
                Ordering::Less => chinese.push('进'),
                _ => chinese.push('退'),
            }
            chinese.push(verticals[mv.to_x]);
        }
        'R' | 'C' => {
            // 判断是否有纵向重叠情况
            let other_ys = overlap_piece_y(board, mv.from_x, mv.from_y, piece);
            if other_ys.is_empty() {
                // 非重叠情况
                chinese.push(get_piece_name(piece));
                chinese.push(verticals[mv.from_x]);
            } else {
                // 重叠, 判断前后
                if mv.from_y > other_ys[0] {
                    chinese.push('后')
                } else {
                    chinese.push('前')
                }
                chinese.push(get_piece_name(piece));
            }
            match mv.from_y.cmp(&mv.to_y) {
                Ordering::Less => {
                    // 退
                    let step = 9 - mv.to_y + mv.from_y;
                    chinese.push('退');
                    chinese.push(verticals[step]);
                }
                Ordering::Greater => {
                    // 进
                    let step = 9 - mv.from_y + mv.to_y;
                    chinese.push('进');
                    chinese.push(verticals[step]);
                }
                Ordering::Equal => {
                    // 平
                    chinese.push('平');
                    chinese.push(verticals[mv.to_x]);
                }
            }
        }
        'r' | 'c' => {
            // 判断是否有纵向重叠情况
            let other_ys = overlap_piece_y(board, mv.from_x, mv.from_y, piece);
            if other_ys.is_empty() {
                // 非重叠情况
                chinese.push(get_piece_name(piece));
                chinese.push(verticals[mv.from_x]);
            } else {
                // 重叠, 判断前后
                if mv.from_y > other_ys[0] {
                    chinese.push('前')
                } else {
                    chinese.push('后')
                }
                chinese.push(get_piece_name(piece));
            }
            match mv.from_y.cmp(&mv.to_y) {
                Ordering::Less => {
                    // 进
                    let step = mv.to_y - mv.from_y - 1;
                    chinese.push('进');
                    chinese.push(verticals[step]);
                }
                Ordering::Greater => {
                    // 退
                    let step = mv.from_y - mv.to_y - 1;
                    chinese.push('退');
                    chinese.push(verticals[step]);
                }
                Ordering::Equal => {
                    // 平
                    chinese.push('平');
                    chinese.push(verticals[mv.to_x]);
                }
            }
        }
        'N' => {
            // 判断是否有纵向重叠情况
            let other_ys = overlap_piece_y(board, mv.from_x, mv.from_y, piece);

            if other_ys.is_empty() {
                chinese.push(get_piece_name(piece));
                chinese.push(verticals[mv.from_x]);
            } else {
                if mv.from_y > other_ys[0] {
                    chinese.push('后');
                } else {
                    chinese.push('前');
                }
                chinese.push(get_piece_name(piece));
            }

            if mv.from_y < mv.to_y {
                chinese.push('退');
            } else {
                chinese.push('进');
            }
            chinese.push(verticals[mv.to_x]);
        }
        'n' => {
            // 判断是否有纵向重叠情况
            let other_ys = overlap_piece_y(board, mv.from_x, mv.from_y, piece);
            if other_ys.is_empty() {
                chinese.push(get_piece_name(piece));
                chinese.push(verticals[mv.from_x]);
            } else {
                if mv.from_y > other_ys[0] {
                    chinese.push('前');
                } else {
                    chinese.push('后');
                }
                chinese.push(get_piece_name(piece));
            }

            if mv.from_y < mv.to_y {
                chinese.push('进')
            } else {
                chinese.push('退')
            }
            chinese.push(verticals[mv.to_x]);
        }
        'P' => {
            // 判断是否有纵向重叠情况
            let mut other_ys = overlap_piece_y(board, mv.from_x, mv.from_y, piece);
            if other_ys.is_empty() {
                chinese.push(get_piece_name(piece));
                chinese.push(verticals[mv.from_x]);
            } else {
                // 判断其他纵线上是否有重叠
                if let Some(other_xys) = overlap_piece_xy(board, mv.from_x, piece) {
                    // 其他纵线有重叠
                    for y in &mut other_ys {
                        *y = (mv.from_x + 10) * 100 - *y;
                    }

                    for (x, ys) in &other_xys {
                        for y in ys {
                            other_ys.push((x + 10) * 100 - y);
                        }
                    }
                    // 降序
                    let value = (mv.from_x + 10) * 100 - mv.from_y;
                    other_ys.push(value);
                    other_ys.sort_by(|a, b| b.cmp(a));
                    let seq = other_ys.iter().position(|&v| v == value).map(|i| i + 1).unwrap();
                    chinese.push(verticals[9 - seq]);
                } else if other_ys.len() > 1 {
                    // 找出当前纵向重叠数量
                    let mut num = 1;
                    for y in other_ys {
                        if mv.from_y < y {
                            break;
                        }
                        num += 1;
                    }
                    chinese.push_str(num.to_string().as_str());
                } else {
                    // 只有前后
                    if mv.from_y > other_ys[0] {
                        chinese.push('后');
                    } else {
                        chinese.push('前');
                    }
                }
                chinese.push(get_piece_name(piece));
            }

            if mv.from_y == mv.to_y {
                // 平
                chinese.push('平');
                chinese.push(verticals[mv.to_x]);
            } else {
                // 进
                chinese.push('进');
                chinese.push(verticals[8]);
            }
        }
        'p' => {
            // 判断是否有纵向重叠情况
            let mut other_ys = overlap_piece_y(board, mv.from_x, mv.from_y, piece);
            if other_ys.is_empty() {
                chinese.push(get_piece_name(piece));
                chinese.push(verticals[mv.from_x]);
            } else {
                // 判断其他纵线上是否有重叠
                if let Some(other_xys) = overlap_piece_xy(board, mv.from_x, piece) {
                    // 其他纵线有重叠
                    for y in &mut other_ys {
                        *y += mv.from_x * 100;
                    }

                    for (x, ys) in &other_xys {
                        for y in ys {
                            other_ys.push(x * 100 + y);
                        }
                    }
                    // 升序
                    let value = mv.from_x * 100 + mv.from_y;
                    other_ys.push(value);
                    other_ys.sort_by(|a, b| b.cmp(a));
                    let seq = other_ys.iter().position(|&v| v == value).unwrap();
                    chinese.push(verticals[seq]);
                } else if other_ys.len() > 1 {
                    // 找出当前纵向重叠数量
                    let mut num = 0;
                    for y in other_ys {
                        if mv.from_y > y {
                            break;
                        }
                        num += 1;
                    }
                    chinese.push_str(num.to_string().as_str());
                } else {
                    // 只有前后
                    if mv.from_y > other_ys[0] {
                        chinese.push('前');
                    } else {
                        chinese.push('后');
                    }
                }
                chinese.push(get_piece_name(piece));
            }

            if mv.from_y == mv.to_y {
                // 平
                chinese.push('平');
                chinese.push(verticals[mv.to_x]);
            } else {
                // 进
                chinese.push('进');
                chinese.push(verticals[0]);
            }
        }
        _ => {}
    }

    chinese
}

#[allow(dead_code)]
pub fn fen_to_board(mut fen: &str) -> [[char; 9]; 10] {
    if fen.contains(' ') {
        fen = fen.split_once(' ').unwrap().0
    }
    let mut board = [[' '; 9]; 10];
    let mut rank = 0;
    let mut file = 0;
    for c in fen.chars() {
        match c {
            '1'..='9' => {
                file += c.to_digit(10).unwrap() as usize;
            }
            '/' => {
                rank += 1;
                file = 0;
            }
            _ => {
                board[rank][file] = c;
                file += 1;
            }
        }
    }
    board
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
        let expected_fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RNBAKABNR w";
        assert_eq!(board_fen(&Camp::Red, board), expected_fen);
    }

    #[test]
    fn test_fen_to_board() {
        let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RNBAKABNR";
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
        assert_eq!(fen_to_board(fen), board);
    }

    #[test]
    fn test_chinese() {
        let fen = "2rakab2/9/1cn6/p3p3p/2b2n3/6R2/P3P1c1P/2N1C3C/4N4/2BAKAB2 w";
        let mut board = fen_to_board(fen);
        for pv in ["g4g5", "b7b5", "g5g9", "c5e7", "g9g4", "f9e8", "i2i6", "c7d5"] {
            let notice = board_move_chinese(board, pv);
            board = board_move(board, pv);
            println!("pv: {} => {}", pv, notice);
        }
    }

    #[test]
    fn test_board_check() {
        let board = [
            ['r', 'n', 'b', 'a', 'k', 'a', 'a', 'n', 'r'],
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
        println!("{}", board_check(board))
    }

    #[test]
    fn test_board_diff() {
        let old = [
            ['r', 'n', 'b', 'a', 'k', 'a', 'a', 'n', 'r'],
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
        let new = [
            ['r', 'n', 'b', 'a', 'k', 'a', 'a', 'n', 'r'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'c', ' ', ' ', ' ', ' ', ' ', 'c', ' '],
            ['p', ' ', 'p', ' ', 'p', ' ', 'p', ' ', 'p'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['P', ' ', 'P', ' ', ' ', 'P', 'P', ' ', 'P'],
            [' ', 'C', ' ', ' ', 'C', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['R', 'N', 'B', 'A', 'K', 'A', 'B', 'N', 'R'],
        ];
        let (changed, state) = board_diff(old, new);
        println!("{:?} {:?}", changed, state);
    }

    #[test]
    fn test_board_map() {
        let board = [
            ['r', 'n', 'b', 'a', 'k', 'a', 'a', 'n', 'r'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'c', ' ', ' ', ' ', ' ', ' ', 'c', ' '],
            ['p', ' ', 'p', ' ', 'p', ' ', 'p', ' ', 'p'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['P', ' ', 'P', ' ', 'P', ' ', 'P', ' ', 'P'],
            [' ', 'C', ' ', ' ', 'C', ' ', ' ', ' ', ' '],
            ['R', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', 'N', 'B', 'A', 'K', 'A', 'B', 'N', 'R'],
        ];
        println!("{:?}", board_map(board))
    }

    #[test]
    fn test_board_fix() {
        let mut board: [[char; 9]; 10] = [
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
        board_fix(&Camp::Black, &mut board);
        println!("{:?}", board)
    }
}
