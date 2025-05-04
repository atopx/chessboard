use std::fs;
use std::sync::Mutex;
use std::sync::OnceLock;

use std::path::Path;
use std::path::PathBuf;
use tracing::Level;
use tracing_appender::non_blocking;
use tracing_appender::rolling;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry;
use tracing_subscriber::EnvFilter;

#[cfg(target_os = "windows")]
fn log_dir(home_dir: &Path) -> PathBuf {
    home_dir
        .join("AppData")
        .join("Local")
        .join("xqlink")
        .join("logs")
}

#[cfg(target_os = "macos")]
fn log_dir(home_dir: &Path) -> PathBuf {
    home_dir.join("Library").join("Logs").join("xqlink")
}

#[cfg(target_os = "linux")]
fn log_dir(home_dir: &Path) -> PathBuf {
    home_dir
        .join(".local")
        .join("share")
        .join("xqlink")
        .join("logs")
}

static APPENDER_GUARD: OnceLock<Mutex<Option<tracing_appender::non_blocking::WorkerGuard>>> =
    OnceLock::new();

/// 初始化tracing库，设置全局订阅者
pub fn init_tracer(level: Level, home_dir: &std::path::Path) {
    // 创建环境过滤器
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // 默认过滤级别设置
        let filter_str = format!(
            "{},xqlink={},xqlink_lib={},ort=warn,tower_http=warn,hyper=warn,hyper_util=warn,xcap=warn",
            level.as_str(),
            level.as_str(),
            level.as_str()
        );
        EnvFilter::new(filter_str)
    });

    let log_dir = log_dir(home_dir);

    if !log_dir.exists() {
        fs::create_dir_all(&log_dir).expect("无法创建日志目录");
    }

    // 设置每日滚动日志文件
    let file_appender = rolling::daily(log_dir, "runtime.log");
    let (non_blocking_file, _guard) = non_blocking(file_appender);

    // 保存guard以确保日志写入器保持活跃
    let _unused = APPENDER_GUARD
        .get_or_init(|| Mutex::new(Some(_guard)))
        .lock()
        .unwrap();

    // 创建控制台输出层
    let console_layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(true)
        .compact();

    // 创建文件输出层
    let file_layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(false)
        .with_writer(non_blocking_file)
        .compact();

    // 组装并设置订阅者
    registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .init();
}
