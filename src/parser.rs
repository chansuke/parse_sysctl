//! Parse `sysctl.conf` file.
/// See the detail: https://man7.org/linux/man-pages/man5/sysctl.conf.5.html
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use glob::glob;
use log::{error, info};

use crate::config::SYSCTL_PATHS;
use crate::errors::ParseSysctlError;
use crate::types::{Line, SysCtl};

fn load_sysctl_files(path: &str) -> Result<Vec<PathBuf>, ParseSysctlError> {
    info!("Loading `sysctl.conf` files");

    let paths: Vec<PathBuf> = glob(path)?.filter_map(Result::ok).collect();

    Ok(paths)
}

fn extract_lines(files: Vec<PathBuf>) -> Result<Vec<Line>, ParseSysctlError> {
    info!("Extracting eatch line from file");

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

fn load() -> Result<Vec<Line>, ParseSysctlError> {
    info!("Loading the sysctl configuration file");

    let sysctl_files: Vec<PathBuf> = SYSCTL_PATHS
        .into_iter()
        .flat_map(|path| load_sysctl_files(path).unwrap())
        .collect();

    extract_lines(sysctl_files)
}

pub fn parse() {
    info!("Parsing the sysctl configuration file");

    match load() {
        Ok(lines) => {
            for line in lines {
                match SysCtl::parse_line(&line.0) {
                    Some(SysCtl::Comment(comment)) => {
                        info!("{}", comment);
                    }
                    Some(SysCtl::KeyValue(map)) => {
                        for (key, value) in map {
                            info!("{} = {}", key, value);
                        }
                    }
                    None => {
                        info!("No matched");
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to load the configuration file: {}", e);
        }
    }
}
