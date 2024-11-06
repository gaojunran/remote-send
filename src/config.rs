use std::{fs, io};
use std::path::PathBuf;
use s3::{Bucket, Region};
use s3::error::S3Error;
use serde::Deserialize;
use thiserror::Error;

enum NameConflictMode {  // As S3 doesn't allow files with the same name, we need to handle name conflict
    Rename, // rename the file with a postfix (default)
    Overwrite, // overwrite the file with the same name
    Error, // return an error
}

pub(crate) struct Config {
    pub(crate) bucket: Box<Bucket>,
    pub(crate) prefix: String,
    pub(crate) name_conflict_mode: NameConflictMode,
    pub(crate) file_location: PathBuf,
    pub(crate) delimiter: String
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    s3: TomlS3,
    rule: TomlRule,
}

#[derive(Debug, Deserialize)]
struct TomlS3 {
    region: String,
    endpoint: String,
    access_key: String,
    secret_key: String,
    bucket_name: String,
}

#[derive(Debug, Deserialize)]
struct TomlRule {
    prefix: String,
    name_conflict_mode: String,
    file_location: String,
    delimiter: String,
}

#[derive(Debug, Error)]
pub(crate) enum ConfigError{
    #[error("Failed to read config: {0}")]
    Read(io::Error),
    #[error("Failed to parse config: {0}")]
    Parse(toml::de::Error),
    #[error("Failed to initialize s3 bucket: {0}")]
    S3Init(S3Error),
    #[error("Invalid config value in key: {0}, only accept {1}")]
    InvalidValue(String, String),
}

pub(crate) fn init_config() -> Result<Config, ConfigError> {
    // read from toml
    let config_path = "./config.toml";
    let toml_string = fs::read_to_string(config_path)
        .map_err(|err| ConfigError::Read(err))?;
    let toml_config: TomlConfig = toml::from_str(&toml_string)
        .map_err(|err| ConfigError::Parse(err))?;
    let bucket = init_s3_bucket(
        toml_config.s3.bucket_name,
        toml_config.s3.region,
        toml_config.s3.endpoint,
        toml_config.s3.access_key,
        toml_config.s3.secret_key)
        .map_err(|err| ConfigError::S3Init(err))?;
    let prefix = toml_config.rule.prefix;
    // if prefix.starts_with("/") || prefix.ends_with("/"){
    //     return Err(ConfigError::InvalidValue("prefix".to_string(), "a string without leading or trailing slashes".to_string()));
    // }
    let name_conflict_mode = match toml_config.rule.name_conflict_mode.as_str() {
        "rename" => NameConflictMode::Rename,
        "overwrite" => NameConflictMode::Overwrite,
        "error" => NameConflictMode::Error,
        _ => return Err(ConfigError::InvalidValue("name_conflict_mode".to_string(), "rename, overwrite, error".to_string()))
    };
    let file_location;
    if toml_config.rule.file_location == "cwd"{
        file_location = PathBuf::from(std::env::current_dir().unwrap());
    } else {
        file_location = PathBuf::from(toml_config.rule.file_location);
    }
    if !file_location.exists() || !file_location.is_dir() {
        return Err(ConfigError::InvalidValue("file_location".to_string(), "a valid directory path".to_string()))
    }
    Ok(Config {
        bucket,
        prefix,
        name_conflict_mode,
        file_location,
        delimiter: toml_config.rule.delimiter
    })
}

fn init_s3_bucket(bucket_name: String, region: String, endpoint: String,
                  access_key: String, secret_key: String)
                  -> Result<Box<Bucket>, S3Error> {
    let region = Region::Custom {
        region,
        endpoint,
    };
    let credentials = s3::creds::Credentials::new(
        Option::from(access_key.as_str()),
        Option::from(secret_key.as_str()),
        None, None, None);
    let s3 = Bucket::new(&bucket_name, region, credentials?);
    s3
}