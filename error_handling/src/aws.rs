use rusoto_core::Region;
use rusoto_s3::{ListBucketsOutput, S3Client, S3};
use thiserror::Error;

extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::credential::ChainProvider;
use rusoto_core::request::HttpClient;
// use rusoto_core::Region;
// use rusoto_s3::{S3Client, S3};
use std::time::{Duration, Instant};

#[derive(Error, Debug)]
pub enum AwsError {
    #[error("unknown aws error")]
    Unknown { msg: String },

    #[error("GenericError occured")]
    GenericError {
        msg: String,
        source: std::sync::Arc<dyn std::error::Error + Sync + Send>,
    },
    // StdIoError(#[from] std::io::Error),
    #[error(transparent)]
    ListBucketsError(#[from] rusoto_core::RusotoError<rusoto_s3::ListBucketsError>),
}

pub fn s3_list_generic_error() -> Result<ListBucketsOutput, AwsError> {
    return Err(AwsError::Unknown {
        msg: "XYZ".to_string(),
    });
}

pub fn s3_list_generic_error_file() -> Result<std::fs::File, AwsError> {
    let f = std::fs::File::open("hello.txt").map_err(|error| AwsError::GenericError {
        msg: "ABC".to_string(),
        source: std::sync::Arc::new(error),
    })?;
    return Ok(f);
}

pub fn s3_list_3() -> Result<ListBucketsOutput, AwsError> {
    let runtime = tokio::runtime::Runtime::new().map_err(|error| AwsError::GenericError {
        msg: "ABC".to_string(),
        source: std::sync::Arc::new(error),
    })?;
    let mut chain = rusoto_core::credential::ChainProvider::new();
    chain.set_timeout(Duration::from_millis(50));
    let s3client = S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        chain,
        Region::ApSoutheast2,
    );

    let resp = runtime.block_on(s3client.list_buckets())?;
    return Ok(resp);
}

// pub fn s3_list() -> Result<ListBucketsOutput, AwsError> {
//     let runtime = tokio::runtime::Runtime::new()?;
//     let s3_client = S3Client::new(Region::ApSoutheast2);

//     let resp = runtime.block_on(s3_client.list_buckets())?;
//     return Ok(resp);
// }

// pub fn s3_list_2() {
//     let runtime = tokio::runtime::Runtime::new().unwrap();
//     let mut chain = rusoto_core::credential::ChainProvider::new();
//     chain.set_timeout(Duration::from_millis(50));
//     let s3client = S3Client::new_with(
//         HttpClient::new().expect("failed to create request dispatcher"),
//         chain,
//         Region::ApSoutheast2,
//     );

//     let start = Instant::now();
//     println!("Starting up at {:?}", start);

//     match runtime.block_on(s3client.list_buckets()) {
//         Err(e) => println!("Error listing buckets: {}", e),
//         Ok(buckets) => println!("Buckets found: {:?}", buckets),
//     };
//     println!("Took {:?}", Instant::now().duration_since(start));
// }
