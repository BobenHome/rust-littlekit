use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logger() {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");

    fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(file_appender)
        .with_target(false)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .with_ansi(false)
        .with_level(true)
        .pretty()
        .try_init()
        .expect("Failed to set tracing subscriber");
}
