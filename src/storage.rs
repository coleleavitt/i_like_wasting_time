use crate::data::JobApplication;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use thiserror::Error;

const DATA_FILE: &str = "job_applications.json";

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Failed to open {0}")]
    FileOpen(String),

    #[error("Failed to create {0}")]
    FileCreate(String),

    #[error("Failed to parse job data")]
    ParseError,

    #[error("Failed to create backup")]
    BackupError,
}

pub type Result<T> = std::result::Result<T, StorageError>;

/// Loads job applications from the JSON file
pub fn load_jobs() -> Result<Vec<JobApplication>> {
    let path = Path::new(DATA_FILE);

    // If file doesn't exist yet, return empty vector
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)
        .map_err(|_| StorageError::FileOpen(DATA_FILE.to_string()))?;
    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
        .map_err(|e| {
            if e.is_syntax() || e.is_data() {
                StorageError::ParseError
            } else {
                StorageError::Json(e)
            }
        })
}

/// Saves job applications to the JSON file
pub fn save_jobs(jobs: &[JobApplication]) -> Result<()> {
    let file = File::create(DATA_FILE)
        .map_err(|_| StorageError::FileCreate(DATA_FILE.to_string()))?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, jobs)?;
    Ok(())
}

/// Backs up the current data file before significant operations
pub fn backup_data() -> Result<()> {
    let path = Path::new(DATA_FILE);
    if path.exists() {
        let backup_file = format!("{}.backup", DATA_FILE);
        fs::copy(DATA_FILE, backup_file)
            .map_err(|_| StorageError::BackupError)?;
    }
    Ok(())
}
