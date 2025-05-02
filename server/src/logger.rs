use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

/// 初始化tracing库，设置全局订阅者。
pub fn init_tracer(level: Level) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("debug,ort=warn,tower_http=warn,hyper=warn,hyper_util=warn,xcap=warn")
    });

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_max_level(level)
        .with_test_writer()
        .finish();

    // 设置全局默认订阅者
    set_global_default(subscriber).expect("Setting default subscriber failed");
}
