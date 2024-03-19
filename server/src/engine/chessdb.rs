use std::time::Duration;

use super::{QueryRecords, QueryState};

const URL: &str = "http://www.chessdb.cn/chessdb.php";
const REFER: &str = "https://www.chessdb.cn/query/";
const AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36";

pub async fn query(fen: &str) -> QueryRecords {
    let mut records = super::QueryRecords::default();
    let resp = reqwest::Client::new()
        .get(URL)
        .header("User-Agent", AGENT)
        .header("Referer", REFER)
        .query(&[("action", "querypv"), ("board", fen)])
        .timeout(Duration::new(5, 0))
        .send()
        .await;
    match resp {
        Ok(resp) => match resp.text().await.unwrap().as_str() {
            "" | "unknown" => records.state = QueryState::NotResult,
            "invalid board" => records.state = QueryState::InvalidBoard,
            "checkmate" | "stalemate" => records.checkmate = true,
            text => {
                for pair in text.split(',') {
                    let mut parts = pair.split(':');
                    match parts.next().unwrap_or("") {
                        "score" => records.score = parts.next().unwrap().parse().unwrap_or(0),
                        "depth" => records.depth = parts.next().unwrap().parse().unwrap_or(0),
                        "pv" => {
                            for pv in parts.next().unwrap().split('|') {
                                records.moves.push(pv.to_string());
                            }
                        }
                        _ => {}
                    }
                }
                records.best = true;
                records.source = super::Source::Chessdb;
            }
        },
        Err(e) => {
            println!("{}", e);
            records.state = QueryState::ServerInternalError
        }
    };
    records
}
