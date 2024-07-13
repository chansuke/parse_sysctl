//! Load sysctl configuration files
use crate::errors::ParseSysctlError;
use glob::glob;
use log::info;
use std::path::PathBuf;

/// Search the  `sysctl.conf` files.
pub(crate) fn load_sysctl_files(path: &str) -> Result<Vec<PathBuf>, ParseSysctlError> {
    info!("Loading `sysctl.conf` files");
    let paths: Vec<PathBuf> = glob(path)?.filter_map(Result::ok).collect();
    Ok(paths)
}

/// Load all sysctl configuration files.
pub(crate) fn load_all_sysctl_files(paths: &[&str]) -> Result<Vec<PathBuf>, ParseSysctlError> {
    info!("Loading all sysctl configuration files");
    let sysctl_files: Vec<PathBuf> = paths
        .iter()
        .flat_map(|&path| load_sysctl_files(path).unwrap_or_default())
        .collect();
    Ok(sysctl_files)
}
