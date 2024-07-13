//! Parse `sysctl.conf` file.
/// See the detail: https://man7.org/linux/man-pages/man5/sysctl.conf.5.html
use log::{error, info};

use crate::config::SYSCTL_PATHS;
use crate::errors::ParseSysctlError;
use crate::extractor::extract_lines;
use crate::loader::load_all_sysctl_files;
use crate::types::{Line, SysCtl};

fn load() -> Result<Vec<Line>, ParseSysctlError> {
    let sysctl_paths = load_all_sysctl_files(SYSCTL_PATHS)?;
    extract_lines(sysctl_paths)
}

pub(crate) fn parse() {
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
