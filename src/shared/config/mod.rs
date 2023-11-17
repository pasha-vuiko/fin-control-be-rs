use dotenv::dotenv;

mod errors;
mod types;

pub use errors::ConfigErrors;
pub use types::*;

pub fn get_config() -> Result<AppConfig, ConfigErrors> {
    // fetch ENV vars from the file if exists
    dotenv().map_err(|err| ConfigErrors::ConfigFileError(err.to_string()))?;

    let config =
        envy::from_env::<AppConfig>().map_err(|err| ConfigErrors::ParseError(err.to_string()))?;

    Ok(config)
}
