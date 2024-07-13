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
