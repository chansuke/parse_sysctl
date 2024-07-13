//! Parse `sysctl.conf` file.
/// See the detail: https://man7.org/linux/man-pages/man5/sysctl.conf.5.html
use log::{error, info};

use crate::config::SYSCTL_PATHS;
use crate::errors::ParseSysctlError;
use crate::extractor::extract_lines;
use crate::loader::load_all_sysctl_files;
use crate::types::{Line, RawFilePaths, SysCtl};

fn load() -> Result<Vec<Line>, ParseSysctlError> {
    let sysctl_paths = load_all_sysctl_files(RawFilePaths(SYSCTL_PATHS))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SysCtlConfPaths;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "net.ipv4.ip_forward = 1").unwrap();
        writeln!(file, "# net.ipv4.ip_forward = 0").unwrap();
        writeln!(file, "; net.ipv4.ip_forward = 0").unwrap();
        file.flush().unwrap();

        let paths = SysCtlConfPaths(vec![file.path().to_path_buf()]);
        let lines = extract_lines(paths).unwrap();
        assert_eq!(lines.len(), 3);

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
}
