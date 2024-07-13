/// Default paths for sysctl configuration files
/// Belows are based on the sysctl.conf(5) — Linux manual page.
pub const SYSCTL_PATHS: &[&str] = &[
    "/etc/sysctl.d/*.conf",
    "/run/sysctl.d/*.conf",
    "/usr/local/lib/sysctl.d/*.conf",
    "/usr/lib/sysctl.d/*.conf",
    "/lib/sysctl.d/*.conf",
    "/etc/sysctl.conf",
];
