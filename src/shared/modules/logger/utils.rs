use crate::shared::modules::logger::errors::CreateCrateLogFilterError;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub fn get_crates_log_filter() -> Result<EnvFilter, CreateCrateLogFilterError> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env()?
        .add_directive("hyper::proto=info".parse()?)
        .add_directive("hyper::client=info".parse()?);

    Ok(filter)
}
