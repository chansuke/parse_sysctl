//! Configuration related constants.
/// Default paths for sysctl configuration files
/// Belows are based on the sysctl.conf(5) — Linux manual page.
/// https://man7.org/linux/man-pages/man5/sysctl.conf.5.html#FILES
pub const SYSCTL_PATHS: &[&str] = &[
    "/etc/sysctl.d/*.conf",
    "/run/sysctl.d/*.conf",
    "/usr/local/lib/sysctl.d/*.conf",
    "/usr/lib/sysctl.d/*.conf",
    "/lib/sysctl.d/*.conf",
    "/etc/sysctl.conf",
];
