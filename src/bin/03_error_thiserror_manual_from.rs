//! The From and Into traits:
//!
//! ```
//! trait From<T> {
//!     fn from(something: T) -> Self;
//! }
//!
//! trait Into<T> {
//!     fn into(self) -> T;
//! }
//!
//! impl<T, U> Into<U> for T
//! where
//!     U: From<T>,
//! {
//!     fn into(self) -> U {
//!         U::from(self)
//!     }
//! }
//! ```

use std::fmt;
use std::io::Error;
use thiserror::Error;

#[derive(Debug, Error)]
enum NitriumError {
    #[error("Encountered a problem when registering this device with Nitrium!")]
    RegisteringDevice,
    #[error("Encountered a problem when sending a heartbeat to Nitrium!")]
    Heartbeat,
    #[error("Failed to read hte product key file: {0}")]
    FailedToReadProductKey(std::io::Error),
}

impl From<std::io::Error> for NitriumError {
    fn from(ioe: std::io::Error) -> NitriumError {
        NitriumError::FailedToReadProductKey(ioe)
    }
}

fn main() {
    let result = get_product_key_manual_err();
    // let result = get_product_key_map_err();
    // let result = get_product_key_match_from();
    // let result = get_product_key_try();
    // let result = get_product_key();

    println!("Get Product Key Debug: {:#?}", result);
}

fn get_product_key_manual_err() -> Result<Vec<u8>, NitriumError> {
    let product_key_result = std::fs::read("./product-key.pem");
    let product_key = match product_key_result {
        Ok(data) => data,
        Err(io_error) => return Err(NitriumError::FailedToReadProductKey(io_error)),
    };
    Ok(product_key)
}

// fn get_product_key_map_err() -> Result<Vec<u8>, NitriumError> {
//     let product_key_result = std::fs::read("./product-key.pem");
//     let product_key =
//         product_key_result.map_err(|error| NitriumError::FailedToReadProductKey(error));
//     product_key
// }
//
// fn get_product_key_match_from() -> Result<Vec<u8>, NitriumError> {
//     let product_key = match std::fs::read("./product-key.pem") {
//         Ok(data) => data,
//         Err(ioe) => return Err(NitriumError::from(ioe)),
//         // Alternately, we can use Into
//         // Err(ioe) => return Err(ioe.into()),
//     };
//     Ok(product_key)
// }
//
// fn get_product_key_try() -> Result<Vec<u8>, NitriumError> {
//     let product_key = r#try!(std::fs::read("./product-key.pem"));
//     Ok(product_key)
// }
//
// fn get_product_key() -> Result<Vec<u8>, NitriumError> {
//     let product_key = std::fs::read("./product-key.pem")?;
//     Ok(product_key)
// }
