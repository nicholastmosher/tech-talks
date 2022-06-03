use std::fmt;

// In Rust, most error types are an "enum" with variants describing error scenarios
enum NitriumError {
    RegisteringDevice,
    Heartbeat,
    Io(std::io::Error),
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
            NitriumError::Io(error) => write!(f, "Encountered an IO error: {}", error),
        }
    }
}

// The "Debug" trait is similar to "Display", but usually shows the internal details of the type
impl fmt::Debug for NitriumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NitriumError::RegisteringDevice => f.debug_tuple("RegisteringDevice").finish(),
            NitriumError::Heartbeat => f.debug_tuple("Heartbeat").finish(),
            NitriumError::Io(e) => f.debug_tuple("Io").field(e).finish(),
        }
    }
}

fn main() {
    let error = NitriumError::RegisteringDevice;

    println!("Nitrium Error Display: {}", error);
    println!("Nitrium Error Debug: {:?}", error);
}
