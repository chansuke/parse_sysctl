//! Extractor module
use crate::errors::ParseSysctlError;
use crate::types::Line;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Extracts each lines from `sysctl.conf` files.
pub(crate) fn extract_lines(files: Vec<PathBuf>) -> Result<Vec<Line>, ParseSysctlError> {
    info!("Extracting each line from files");
    let lines = files
        .into_iter()
        .filter_map(|path| {
            File::open(path).ok().map(|file| {
                BufReader::new(file)
                    .lines()
                    .map_while(Result::ok)
                    .map(|line| Line(line.trim().to_owned()))
            })
        })
        .flatten()
        .collect();
    Ok(lines)
}
