use crate::shared::config::LogLevel;
use tracing::metadata::LevelFilter;

impl From<&LogLevel> for LevelFilter {
    fn from(value: &LogLevel) -> Self {
        match value {
            LogLevel::Error => Self::ERROR,
            LogLevel::Warn => Self::WARN,
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
            LogLevel::Silent => Self::OFF,
        }
    }
}
