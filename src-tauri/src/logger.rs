use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// 初始化tracing库，设置全局订阅者。
pub fn init_tracer(level: Level) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        // 可以选择使用环境变量来控制日志级别
        // 对于测试环境，使用.with_test_writer()
        // 对于生产环境，可以省略此行，或将日志输出到文件等其他位置
        .with_test_writer()
        .finish();

    // 设置全局默认订阅者
    set_global_default(subscriber).expect("Setting default subscriber failed");
}
