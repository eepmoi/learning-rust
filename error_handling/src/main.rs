// use crate::file::*;
mod aws;
mod file;
use crate::aws::*;
use crate::file::*;

fn main() {
    // s3_list_main()
    // s3_list_3_main();
    // s3_list_generic_error().unwrap();
    s3_list_generic_error_main();
    // no handling of error, just panic
    // open_file().unwrap();
}

// fn s3_list_main() {
//     s3_list().unwrap();
//     println!("s3_list_main");
// }

// fn s3_list_2_main() {
//     s3_list_2();
//     println!("s3_list_2_main");
// }

fn s3_list_generic_error_main() {
    match s3_list_generic_error() {
        Ok(buckets) => println!("Buckets found: {:?}", buckets),
        // Err(e) => println!("Error listing buckets: {}", e),
        // Err(e) => println!("Error listing buckets: {:?}", e),
        Err(e) => println!("Error listing buckets: {:?}", e), // Err(andy) => println!("Unknown error {:?}", andy),
    };
    println!("s3_list_generic_error_main");
}

fn s3_list_3_main() {
    match s3_list_3() {
        Ok(buckets) => println!("Buckets found: {:?}", buckets),
        // Err(e) => println!("Error listing buckets: {}", e),
        // Err(e) => println!("Error listing buckets: {:?}", e),
        // Err(AwsError::ListBucketsError(rusoto_core::RusotoError::Credentials(e))) => {
        Err(AwsError::ListBucketsError(rusoto_core::RusotoError::Credentials(e))) => {
            println!("Error listing buckets: {:?}", e)
        }
        Err(andy) => println!("Unknown error {:?}", andy),
    };
    println!("s3_list_3_main");
}

fn open_file_main() {
    match open_file() {
        Ok(f) => (),
        // Err(e) => println!("File Error {:?}", e),
        // Err(e2) => match e2 {
        //     FileError::StdIoError(e3) => println!("File Error Kind {:?}", e3.kind()),
        Err(FileError::StdIoError(e3)) => println!("File Error Kind {:?}", e3.kind()),
    }

    println!("Hello world");
}
