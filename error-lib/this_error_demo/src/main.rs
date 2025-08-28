use std::{error::Error, io};

use thiserror::Error;

#[derive(Debug, Error)]
enum ParserError {
    // Use #[from] to automatically implement From<io::Error> and implicitly set #[source]
    #[error("Failed to read file: {0}")]
    IoError(#[from] io::Error),

    // Use #[from] to automatically implement From<serde_json::Error> and implicitly set #[source]
    #[error("Failed to parse JSON")]
    JsonError(#[from] serde_json::Error),

    // Manually specify #[source], do not automatically implement From trait
    #[error("Data validation failed")]
    InvalidData {
        #[source] // Explicitly mark the source error
        cause: InvalidDataError,
    },
}

// Custom business logic error

#[derive(Debug, Error)]
#[error("Invalid data: {0}")]
struct InvalidDataError(String);

fn read_file(path: &str) -> Result<String, ParserError> {
    // ? Automatically convert io::Error into ParserError::IoError

    let content = std::fs::read_to_string(path)?;

    Ok(content)
}

fn validate_data(data: &str) -> Result<(), InvalidDataError> {
    if data.is_empty() {
        return Err(InvalidDataError("Data cannot be empty".into()));
    }

    Ok(())
}

fn process_data(data: &str) -> Result<(), ParserError> {
    validate_data(data).map_err(|e| ParserError::InvalidData { cause: e })?;

    Ok(())
}

fn main() {
    // Test IoError (automatic From conversion)
    let io_err = read_file("nonexistent.txt").unwrap_err();
    println!("IO Error Print: {}", io_err);
    println!("IO Error Debug: {:?}", io_err);
    println!("Source: {:?}", io_err.source()); // Can get the underlying io::Error

    // Test InvalidData (manual #[source] wrapping)
    let validation_err = process_data("").unwrap_err();
    println!("Validation Error Print: {}", validation_err);
    println!("Validation Error Debug: {:?}", validation_err);
    println!("Source: {:?}", validation_err.source()); // Can get InvalidDataError
}
