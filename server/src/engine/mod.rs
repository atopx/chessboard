pub mod chessdb;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::task;

#[derive(Debug, serde::Serialize, Default)]
pub struct QueryRecords {
    pub best: bool,         // 是否最佳行棋
    pub checkmate: bool,    // 是否将死
    pub depth: usize,       // 深度
    pub score: usize,       // 得分
    pub time: usize,        // 时间
    pub moves: Vec<String>, // 思考
    pub state: QueryState,  // 状态
    pub source: Source,     // 来源
}

#[derive(Debug, serde::Serialize, Default)]
pub enum Source {
    #[default]
    Engine,
    Chessdb,
}

#[derive(Debug, serde::Serialize, Default)]
pub enum QueryState {
    #[default]
    Success,
    NotResult,
    InvalidBoard,
    ServerInternalError, // 内部错误
}

struct Engine {
    pub sender: mpsc::Sender<String>,
    pub receiver: mpsc::Receiver<QueryRecords>,
    tx: mpsc::Sender<QueryRecords>,
}

impl Engine {
    async fn setoption(&self, key: &str, value: &str) {
        self.sender
            .send(format!("setoption name {} value {}", key, value));
    }

    async fn go(&self, fen: &str, depth: usize, time: usize) -> bool {
        // 先查询云库
        let result = chessdb::query(fen).await;

        match result.state {
            QueryState::Success => {
                // 发送查询结果到队列
                self.tx.send(result).await.unwrap();
                true
            }
            QueryState::InvalidBoard => false,
            QueryState::ServerInternalError | QueryState::NotResult => {
                // 查询云库失败调用引擎
                self.sender
                    .send(format!("position fen {}", fen))
                    .await
                    .unwrap();
                self.sender
                    .send(format!("go depth {} time {}", depth, time))
                    .await
                    .unwrap();
                true
            }
        }
    }

    async fn new(path: &str) -> Self {
        let mut cmd = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = cmd.stdout.take().unwrap();
        let mut stdin = cmd.stdin.take().unwrap();
        let (sender, mut rx) = mpsc::channel::<String>(1);
        let (tx, receiver) = mpsc::channel::<QueryRecords>(1);
        let tx_clone = tx.clone();
        task::spawn(async move {
            // 监听引擎输出
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Some(line) = lines.next_line().await.unwrap() {
                // 解析并发送结果到队列
                if let Some(result) = parse_line(line).await {
                    tx_clone.send(result).await.unwrap();
                }
            }
        });

        task::spawn(async move {
            // 监听UCI命令, 调用UCI引擎
            while let Some(msg) = rx.recv().await {
                stdin.write_all(msg.as_bytes()).await.unwrap();
                stdin.write_all(b"\n").await.unwrap();
                stdin.flush().await.unwrap();
            }
        });

        Self {
            tx,
            sender,
            receiver,
        }
    }
}

async fn parse_line(line: String) -> Option<QueryRecords> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query() {
        let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C2C4/9/RNBAKABNR b";
        let result = chessdb::query(fen).await;
        println!("{:?}", result);
    }
}
