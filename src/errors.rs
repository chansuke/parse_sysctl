//! Custom errors.
use thiserror::Error;

use glob::PatternError;

#[derive(Error, Debug)]
pub enum ParseSysctlError {
    #[error("Failed read from file: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Failed to match glob pattern with sysctl.conf type")]
    PatternError(#[from] PatternError),
    #[error("Failed extract the lines from file")]
    ExtractLinesError,
}
