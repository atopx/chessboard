pub mod chessdb;
use std::fmt;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use tracing::debug;
use tracing::trace;

#[derive(Debug, serde::Serialize, Default, Clone)]
pub struct QueryResult {
    pub depth: usize,       // 深度
    pub score: isize,       // 得分
    pub time: usize,        // 时间
    pub pvs: Vec<String>,   // 思考(iccs)
    pub moves: Vec<String>, // 思考(chinese)
    pub state: QueryState,  // 状态
    pub source: String,     // 来源
}

const SOURCE_ENGINE: &str = "引擎";

#[derive(Debug, serde::Serialize, Default, Clone)]
pub enum QueryState {
    Success,
    #[default]
    NotResult,
    InvalidBoard,
    ServerInternalError, // 内部错误
}

pub struct Engine {
    chessdb: bool,
    stdin: Box<dyn Write>,
    stdout: Box<dyn BufRead>,
    child: std::process::Child, // 添加子进程字段
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}

impl Engine {
    pub fn new(libs: &Path) -> Self {
        #[cfg(target_os = "windows")]
        use std::os::windows::process::CommandExt;

        #[cfg(target_os = "windows")]
        let cmd = libs.join("pikafish-windows.exe");

        #[cfg(target_os = "linux")]
        let cmd = libs.join("pikafish-linux");

        #[cfg(target_os = "macos")]
        let cmd = libs.join("pikafish-macos");

        #[cfg(target_os = "windows")]
        let mut process = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .spawn()
            .expect("Unable to run engine");

        println!("cmd {:?}", cmd);

        #[cfg(not(target_os = "windows"))]
        let mut process = Command::new(cmd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Unable to run engine");

        let nnue = libs.join("pikafish.nnue");

        let stdin = process.stdin.take().unwrap();
        let stdout = process.stdout.take().unwrap();
        let buffer = BufReader::new(stdout);

        let mut eng =
            Engine { chessdb: false, stdin: Box::new(stdin), stdout: Box::new(buffer), child: process };
        eng.setoption("EvalFile", nnue.display());
        eng.setoption("Sixty Move Rule", false);
        eng
    }

    pub fn set_chessdb(&mut self, open: bool) { self.chessdb = open; }

    fn write_command<A: fmt::Display>(&mut self, args: A) {
        writeln!(self.stdin, "{}", args).expect("write command error");
        self.stdin.flush().expect("write command flush error");
        debug!("{}", args);
    }

    pub fn set_show_wdl(&mut self, open: bool) { self.setoption("UCI_ShowWDL", open); }

    pub fn set_threads(&mut self, num: usize) { self.setoption("Threads", num); }

    pub fn set_hash(&mut self, size: usize) { self.setoption("Hash", size); }

    pub fn setoption<T: fmt::Display>(&mut self, name: &str, value: T) {
        self.write_command(format!("setoption name {} value {}", name, value))
    }

    pub fn position(&mut self, fen: &str) { self.write_command(format!("position fen {}", fen)) }

    fn read_line(&mut self) -> String {
        let mut line = String::new();
        self.stdout.read_line(&mut line).unwrap();
        trace!("line::{}", line);
        line.trim().to_string()
    }

    fn parse_line(&self, line: String, result: &mut QueryResult) {
        let mut iter = line.split_whitespace();
        result.source = SOURCE_ENGINE.to_string();
        loop {
            if let Some(key) = iter.next() {
                match key {
                    "depth" => {
                        result.depth = iter.next().unwrap().parse().unwrap();
                    }
                    "time" => {
                        result.time = iter.next().unwrap().parse().unwrap();
                    }
                    "score" => match iter.next().unwrap() {
                        "cp" => {
                            result.score = iter.next().unwrap().parse().unwrap();
                        }
                        "mate" => {
                            let round: isize = iter.next().unwrap().parse().unwrap();
                            result.score = if round > 0 { 30000 - round } else { -(30000 + round) };
                        }
                        _ => {}
                    },
                    "pv" => loop {
                        if let Some(pv) = iter.next() {
                            result.pvs.push(pv.to_string());
                            continue;
                        }
                        break;
                    },
                    _ => {}
                }
                continue;
            }
            break;
        }
    }

    fn bestmove(&mut self, depth: usize, time: usize) -> String {
        self.write_command(format!("go depth {} movetime {}", depth, time));
        let mut pre_line = String::new();
        loop {
            let line = self.read_line();
            if line.starts_with("bestmove") {
                trace!("{}", pre_line);
                break;
            }
            pre_line = line;
        }
        pre_line
    }

    pub async fn go(&mut self, fen: &str, depth: usize, time: usize) -> Option<QueryResult> {
        // 先查询云库
        let mut result = if self.chessdb { chessdb::query(fen).await } else { QueryResult::default() };
        match result.state {
            QueryState::Success => Some(result),
            QueryState::InvalidBoard => None,
            QueryState::ServerInternalError | QueryState::NotResult => {
                // 查询云库失败调用引擎
                self.position(fen);
                let best_line = self.bestmove(depth, time);
                self.parse_line(best_line, &mut result);
                Some(result)
            }
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[cfg(test)]
mod tests {
    use std::path;

    use tracing::info;
    use tracing::Level;

    use super::*;
    use crate::logger;

    #[tokio::test]
    async fn test_query() {
        logger::init_tracer(Level::TRACE);
        let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RNBAKABNR b";
        let result = chessdb::query(fen).await;
        info!("{:?}", result);
    }
    #[tokio::test]
    async fn test_engine() {
        logger::init_tracer(Level::TRACE);
        let fen = "4k4/9/6r2/9/9/9/9/9/4A4/4K4 w";
        let libs = path::PathBuf::from("/Users/atopx/script/chessboard/libs");
        let mut eng = Engine::new(&libs);
        let records = eng.go(fen, 10, 1000).await;
        info!("{:?}", records);
    }
}
