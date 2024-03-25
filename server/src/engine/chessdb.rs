use super::{QueryResult, QueryState};
use std::time::Duration;

const URL: &str = "http://www.chessdb.cn/chessdb.php";
const REFER: &str = "https://www.chessdb.cn/query/";
const AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36";
const SOURCE_CHESSDB: &str = "云库";
pub fn query(fen: &str) -> QueryResult {
    let mut records = super::QueryResult::default();
    let resp = reqwest::blocking::Client::new()
        .get(URL)
        .header("User-Agent", AGENT)
        .header("Referer", REFER)
        .query(&[("action", "querypv"), ("board", fen)])
        .timeout(Duration::new(5, 0))
        .send();
    match resp {
        Ok(resp) => match resp.text().unwrap().as_str() {
            "" | "unknown" => records.state = QueryState::NotResult,
            "invalid board" | "checkmate" | "stalemate" => records.state = QueryState::InvalidBoard,
            text => {
                for pair in text.split(',') {
                    let mut parts = pair.split(':');
                    match parts.next().unwrap_or("") {
                        "score" => records.score = parts.next().unwrap().parse().unwrap_or(0),
                        "depth" => records.depth = parts.next().unwrap().parse().unwrap_or(0),
                        "pv" => {
                            let mut pv_text = parts.next().unwrap();
                            pv_text = pv_text.strip_suffix('\0').unwrap();
                            for pv in pv_text.split('|') {
                                records.moves.push(pv.to_string());
                            }
                        }
                        _ => {}
                    }
                }
                records.state = QueryState::Success;
                records.source = SOURCE_CHESSDB.to_string();
            }
        },
        Err(e) => {
            println!("{}", e);
            records.state = QueryState::ServerInternalError
        }
    };
    records
}
