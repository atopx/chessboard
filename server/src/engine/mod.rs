pub mod chessdb;
use std::fmt::Display;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
mod command;

use tracing::debug;
use tracing::trace;

pub struct SearchParams {
    pub fen: String,
    pub depth: usize,
    pub time: usize,
    pub chessdb_enabled: bool,
    pub chessdb_timeout: u64,
}

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
    stdin: Box<dyn Write>,
    stdout: Box<dyn BufRead>,
    child: std::process::Child, // 添加子进程字段
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}

impl Engine {
    pub fn new(libs: &Path) -> Self {
        let mut child = command::new(libs);

        let nnue = libs.join("pikafish.nnue");

        let stdin = Box::new(child.stdin.take().unwrap());
        let stdout = Box::new(BufReader::new(child.stdout.take().unwrap()));

        let mut eng = Engine {
            stdin,
            stdout,
            child,
        };
        eng.setoption("EvalFile", nnue.display());
        eng.setoption("Sixty Move Rule", false);
        eng
    }

    fn write_command<A: Display>(&mut self, args: A) {
        writeln!(self.stdin, "{}", args).expect("write command error");
        self.stdin.flush().expect("write command flush error");
        debug!("{}", args);
    }

    pub fn set_show_wdl(&mut self, open: bool) {
        self.setoption("UCI_ShowWDL", open);
    }

    pub fn set_threads(&mut self, num: usize) {
        self.setoption("Threads", num);
    }

    pub fn set_hash(&mut self, size: usize) {
        self.setoption("Hash", size);
    }

    pub fn setoption<T: Display>(&mut self, name: &str, value: T) {
        self.write_command(format!("setoption name {} value {}", name, value))
    }

    pub fn position(&mut self, fen: &str) {
        self.write_command(format!("position fen {}", fen))
    }

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
                            result.score = if round > 0 {
                                30000 - round
                            } else {
                                -(30000 + round)
                            };
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

    pub async fn search(&mut self, params: &SearchParams) -> Option<QueryResult> {
        let mut result = if params.chessdb_enabled {
            // 先查询云库
            chessdb::query(&params.fen, params.chessdb_timeout).await
        } else {
            QueryResult::default()
        };

        match result.state {
            QueryState::Success => Some(result),
            QueryState::InvalidBoard => None,
            QueryState::ServerInternalError | QueryState::NotResult => {
                // 查询云库失败调用引擎
                self.position(&params.fen);
                let best_line = self.bestmove(params.depth, params.time);
                self.parse_line(best_line, &mut result);
                Some(result)
            }
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.write_command("quit");
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
        logger::init_tracer(Level::TRACE, &std::path::PathBuf::from("."));
        let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RNBAKABNR b";
        let result = chessdb::query(fen, 10).await;
        info!("{:?}", result);
    }
    #[tokio::test]
    async fn test_engine() {
        logger::init_tracer(Level::TRACE, &std::path::PathBuf::from("."));
        let fen = "4k4/9/6r2/9/9/9/9/9/4A4/4K4 w";
        let libs = path::PathBuf::from("/Users/atopx/script/chessboard/libs");
        let mut eng = Engine::new(&libs);
        let records = eng
            .search(&SearchParams {
                fen: fen.to_string(),
                depth: 10,
                time: 5,
                chessdb_enabled: false,
                chessdb_timeout: 10,
            })
            .await;
        info!("{:?}", records);
    }
}
