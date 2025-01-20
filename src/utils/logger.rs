use std::path::PathBuf;
use time::macros::format_description;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, fmt::time::OffsetTime, EnvFilter};

pub fn init_logger() {
    // 获取用户家目录
    let home_dir = std::env::var("HOME").expect("Failed to get HOME directory");
    let log_dir = PathBuf::from(home_dir)
        .join("workspace")
        .join("applications")
        .join("logs")
        .join("rust-littlekit");

    // 确保日志目录存在
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir).expect("Failed to create logs directory");
    }

    let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "app.log");

    // 创建自定义的环境过滤器
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("info")
            .add_directive("sqlx=debug".parse().unwrap())
            .add_directive("tower_http=debug".parse().unwrap())
    });

    // 自定义时间格式
    let timer = OffsetTime::new(
        time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );

    fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(file_appender)
        .with_timer(timer)
        .with_target(true)
        .with_thread_ids(false)
        .with_line_number(false)
        .with_file(false)
        .with_ansi(false)
        .with_level(true)
        .event_format(
            fmt::format()
                .with_level(true)
                .with_target(true)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false)
                .compact(),
        )
        .try_init()
        .expect("Failed to set tracing subscriber");
}
