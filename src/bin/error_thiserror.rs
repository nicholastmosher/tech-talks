use std::fmt;
use thiserror::Error;

// In Rust, most error types are an "enum" with variants describing error scenarios
#[derive(Debug, Error)]
enum NitriumError {
    #[error("Encountered a problem when registering this device with Nitrium!")]
    RegisteringDevice,
    #[error("Encountered a problem when sending a heartbeat to Nitrium!")]
    Heartbeat,
    #[error("Failed to read hte product key file: {0}")]
    FailedToReadProductKey(std::io::Error),
}

fn main() {
    let error = NitriumError::FailedToReadProductKey(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "product key file not found",
    ));

    println!("Nitrium Error Display: {}", error);
    println!("Nitrium Error Debug: {:#?}", error);
}
