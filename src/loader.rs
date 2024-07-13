//! Load sysctl configuration files
use glob::glob;
use log::info;
use std::path::PathBuf;

use crate::errors::ParseSysctlError;
use crate::types::SysCtlConfPaths;

/// Search the  `sysctl.conf` files.
pub(crate) fn load_sysctl_files(path: &str) -> Result<SysCtlConfPaths, ParseSysctlError> {
    info!("Loading `sysctl.conf` files");
    let paths: Vec<PathBuf> = glob(path)?.filter_map(Result::ok).collect();
    Ok(SysCtlConfPaths(paths))
}

/// Load all sysctl configuration files.
pub(crate) fn load_all_sysctl_files(paths: &[&str]) -> Result<SysCtlConfPaths, ParseSysctlError> {
    info!("Loading all sysctl configuration files");
    let sysctl_files: SysCtlConfPaths = paths
        .iter()
        .flat_map(|&path| load_sysctl_files(path).unwrap_or_default())
        .collect();
    Ok(sysctl_files)
}
