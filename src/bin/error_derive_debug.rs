use std::fmt;

// In Rust, most error types are an "enum" with variants describing error scenarios
#[derive(Debug)]
enum NitriumError {
    RegisteringDevice,
    Heartbeat,
    FailedToReadProductKey(std::io::Error),
}

// The "Display" trait describes how to pretty-print a type, similar to Java's "Object.to_string()"
impl fmt::Display for NitriumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NitriumError::RegisteringDevice => write!(
                f,
                "Encountered a problem when registering this device with Nitrium!"
            ),
            NitriumError::Heartbeat => write!(
                f,
                "Encountered a problem when sending a heartbeat to Nitrium!"
            ),
            NitriumError::FailedToReadProductKey(ioe) => {
                write!(f, "Failed to read the product key file: {}", ioe)
            }
        }
    }
}

fn main() {
    let error = NitriumError::FailedToReadProductKey(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "product key file not found",
    ));

    println!("Nitrium Error Display: {}", error);
    println!("Nitrium Error Debug: {:#?}", error);
}
