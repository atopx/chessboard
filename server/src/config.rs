use std::{
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use tracing::{debug, trace};

use crate::STATE;

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    config_path: Option<PathBuf>,
    // trace, debug, info, wran, silent
    pub loglevel: String,
    pub engine_depth: usize,
    pub engine_time: usize,
    pub engine_threads: usize,
    pub engine_hash: usize,
    pub show_wdl: bool,
    pub enable_chessdb: bool,
    pub chessdb_timeout: u64,
    pub timer_interval: u64,
    pub confirm_interval: u64,
}

impl Config {
    pub fn load(path: &PathBuf) -> Self {
        let config_path = path.join("config.json");
        // 打开文件
        let file = File::open(&config_path).unwrap();
        // 创建一个带缓冲区的读取器
        let reader = BufReader::new(file);
        // 解析JSON数据
        let mut config: Config = serde_json::from_reader(reader).unwrap();
        config.config_path = Some(config_path);
        // 返回解析后的数据
        config
    }

    pub fn save(&self) {
        let path = self.config_path.as_ref().unwrap();
        debug!("save config to {:?}", path);
        // 将对象序列化为格式化的JSON字符串
        let json_string = serde_json::to_string_pretty(self).unwrap();

        // 创建一个新文件或打开一个已存在的文件以写入JSON数据
        let mut file = File::create(path).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();
    }

    pub fn get_engine_config(&self) -> EngineConfig {
        EngineConfig {
            depth: self.engine_depth,
            time: self.engine_time as f32 / 1000.0,
            threads: self.engine_threads,
            hash: self.engine_hash,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EngineConfig {
    pub depth: usize,
    pub time: f32,
    pub threads: usize,
    pub hash: usize,
}

#[tauri::command]
pub fn get_engine_config() -> EngineConfig {
    let state = STATE.lock().unwrap();
    trace!("get_engine_config");
    state.config.as_ref().unwrap().get_engine_config()
}

#[tauri::command]
pub fn set_engine_depth(depth: usize) {
    let mut state = STATE.lock().unwrap();
    let config = state.config.as_mut().unwrap();
    config.engine_depth = depth;
    config.save();
    debug!("set_engine_depth: {}", depth);
}

#[tauri::command]
pub fn set_engine_time(time: f32) {
    let mut state = STATE.lock().unwrap();
    let config = state.config.as_mut().unwrap();
    config.engine_time = (time * 1000.0) as usize;
    config.save();
    debug!("set_engine_time: {}", time);
}

#[tauri::command]
pub fn set_engine_threads(num: usize) {
    let mut state = STATE.lock().unwrap();
    let config = state.config.as_mut().unwrap();
    config.engine_threads = num;
    config.save();
    debug!("set_engine_threads: {}", num);
}

#[tauri::command]
pub fn set_engine_hash(size: usize) {
    let mut state = STATE.lock().unwrap();
    let config = state.config.as_mut().unwrap();
    config.engine_hash = size;
    config.save();
    debug!("set_engine_hash: {}", size);
}
