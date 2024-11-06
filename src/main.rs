mod cli;
mod config;
mod s3_action;

use std::error::Error;
use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::{PathBuf, Prefix};
use futures::StreamExt;
use s3::{Bucket, Region};
use s3::error::S3Error;
use s3::serde_types::Object;
use serde::Deserialize;
use tokio::io::AsyncWriteExt;
use cli::cli;
use config::init_config;
use s3_action::{pull_file, push_file};





#[tokio::main]
async fn main() {
    let config = init_config().unwrap();
    cli(&config).await;
    // upload the file to s3
    // let results = config.bucket.list("/rs".to_string(), Some("/".to_string())).await.unwrap();
    //
    // println!("{:#?}", results[0].contents);
    // // pull_file(&config.bucket, &find_latest(&results[0].contents).unwrap().unwrap(), &config.file_location.join("红色经典学习报告.docx")).await.unwrap();
    // push_file(&config.bucket, &mut tokio::fs::File::open("./README.md").await.unwrap(), "rs/README.md").await.unwrap();
    // let results = config.bucket.list("/rs".to_string(), None).await.unwrap();
    // println!("{:#?}", results[0].contents);
}
