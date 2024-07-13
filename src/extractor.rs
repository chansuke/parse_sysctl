//! Extractor module
use crate::errors::ParseSysctlError;
use crate::types::{Line, SysCtlConfPaths};
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Extracts each lines from `sysctl.conf` files.
pub(crate) fn extract_lines(paths: SysCtlConfPaths) -> Result<Vec<Line>, ParseSysctlError> {
    info!("Extracting each line from files");
    let lines = paths
        .0
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_extract_lines() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "net.ipv4.ip_forward = 1").unwrap();
        writeln!(file, "# net.ipv4.ip_forward = 0").unwrap();
        writeln!(file, "; net.ipv4.ip_forward = 0").unwrap();
        file.flush().unwrap();

        let paths = SysCtlConfPaths(vec![file.path().to_path_buf()]);
        let lines = extract_lines(paths).unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].0, "net.ipv4.ip_forward = 1");
        assert_eq!(lines[1].0, "# net.ipv4.ip_forward = 0");
        assert_eq!(lines[2].0, "; net.ipv4.ip_forward = 0");
    }
}
