use futures::stream::StreamExt;
use s3::error::S3Error;
use s3::serde_types::Object;
use s3::Bucket;
use std::io;
use std::path::PathBuf;
use thiserror::Error;
use tokio::io::AsyncWriteExt;

// struct ObjectDetail {
//
// }

#[derive(Debug, Error)]
pub(crate) enum RuntimeError {
    #[error("The bucket is empty.")]
    EmptyBucket(),
    #[error("Failure in S3-related operation: {0}.")]
    S3(S3Error),
    #[error("{0}")]
    Io(io::Error)
}

fn find_latest(objects: &Vec<Object>) -> Result<Option<Object>, S3Error> {
    if objects.is_empty() {
        return Ok(None);
    }
    let latest = objects.iter()
        .filter(|o| o.size > 0)
        .max_by_key(|o| o.last_modified.clone());
    Ok(latest.cloned())
}

fn get_keys(objects: &Vec<Object>) -> Vec<String> {
    objects.iter().map(|o| o.key.clone()).collect()
}

pub(crate) async fn pull_file(
    bucket: &Box<Bucket>,
    object: &Object,
    file_path: &PathBuf
) -> Result<(), RuntimeError> {
    let mut stream = bucket.get_object_stream(object.key.clone()).await
        .map_err(|e| RuntimeError::S3(e))?;
    let mut file = tokio::fs::File::create(file_path).await
        .map_err(|e| RuntimeError::Io(e))?;
    while let Some(chunk) = stream.bytes().next().await {
        let chunk = chunk.map_err(|e| RuntimeError::S3(e))?;
        file.write_all(&chunk).await.map_err(|e| RuntimeError::Io(e))?;
    }
    Ok(())
}

pub(crate) async fn push_file(
    bucket: &Box<Bucket>,
    file: &mut tokio::fs::File,
    target_key: &str,
) -> Result<(), RuntimeError> {
    let mut stream = bucket.put_object_stream(file, target_key).await
        .map_err(|e| RuntimeError::S3(e))?;
    Ok(())
}

pub(crate) async fn list_files(
    delimiter: &Option<String>,
    bucket: &Box<Bucket>,
    prefix: &str,
    // preserved
) -> Result<Vec<Object>, RuntimeError> {
    let result = bucket.list(prefix.to_string(), delimiter.clone()).await
        .map_err(|e| RuntimeError::S3(e))?;
    if result.is_empty() {
        return Err(RuntimeError::EmptyBucket());
    }
    Ok(result[0].contents.clone())
}