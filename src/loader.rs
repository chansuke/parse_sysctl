//! Load sysctl configuration files
use glob::glob;
use log::info;
use std::path::PathBuf;

use crate::errors::ParseSysctlError;
use crate::types::{RawFilePath, RawFilePaths, SysCtlConfPaths};

/// Search the `sysctl.conf` files.
pub(crate) fn load_sysctl_files(path: RawFilePath) -> Result<SysCtlConfPaths, ParseSysctlError> {
    info!("Loading `sysctl.conf` files");
    let paths: Vec<PathBuf> = glob(path.0)?.filter_map(Result::ok).collect();
    Ok(SysCtlConfPaths(paths))
}

/// Load all sysctl configuration files.
pub(crate) fn load_all_sysctl_files(
    paths: RawFilePaths,
) -> Result<SysCtlConfPaths, ParseSysctlError> {
    info!("Loading all sysctl configuration files");
    let sysctl_files: SysCtlConfPaths = paths
        .0
        .iter()
        .flat_map(|&path| load_sysctl_files(RawFilePath(path)).unwrap_or_default())
        .collect();
    Ok(sysctl_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_sysctl_files() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "net.ipv4.ip_forward = 1").unwrap();
        writeln!(file, "# net.ipv4.ip_forward = 0").unwrap();
        writeln!(file, "; net.ipv4.ip_forward = 0").unwrap();
        file.flush().unwrap();

        let paths = RawFilePath(file.path().to_str().unwrap());
        let sysctl_files = load_sysctl_files(paths).unwrap();
        assert_eq!(sysctl_files.0.len(), 1);
    }

    #[test]
    fn test_load_all_sysctl_files() {
        let mut file1 = NamedTempFile::new().unwrap();
        writeln!(file1, "net.ipv4.ip_forward = 1").unwrap();
        writeln!(file1, "# net.ipv4.ip_forward = 0").unwrap();
        writeln!(file1, "; net.ipv4.ip_forward = 0").unwrap();
        file1.flush().unwrap();

        let mut file2 = NamedTempFile::new().unwrap();
        writeln!(file2, "net.ipv4.ip_forward = 1").unwrap();
        writeln!(file2, "# net.ipv4.ip_forward = 0").unwrap();
        writeln!(file2, "; net.ipv4.ip_forward = 0").unwrap();
        file2.flush().unwrap();

        let binding = [
            file1.path().to_str().unwrap(),
            file2.path().to_str().unwrap(),
        ];
        let paths = RawFilePaths(&binding);
        let sysctl_files = load_all_sysctl_files(paths).unwrap();
        assert_eq!(sysctl_files.0.len(), 2);
    }
}
