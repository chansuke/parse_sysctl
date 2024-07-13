//! Defining the types for the application.
use std::collections::HashMap;
use std::path::PathBuf;

type Comment<'a> = &'a str;
pub type KeyValue<'a> = HashMap<&'a str, &'a str>;

/// A types for `sysctl.conf`.
pub enum SysCtl<'a> {
    /// For comment syntax:
    /// - # comment
    /// - ; comment
    Comment(Comment<'a>),
    /// For token/value syntax:
    /// - `token = value`
    KeyValue(KeyValue<'a>),
}

impl<'a> SysCtl<'a> {
    /// Parse a line and return the appropriate SysCtl variant.
    pub fn parse_line(line: &'a str) -> Option<Self> {
        if line.starts_with('#') || line.starts_with(';') {
            Some(SysCtl::Comment(line))
        } else if let Some((key, value)) = line.split_once('=') {
            let mut map = KeyValue::new();
            map.insert(key, value);
            Some(SysCtl::KeyValue(map))
        } else {
            None
        }
    }
}

/// Eatch lines of `sysctl.conf`.
#[derive(Debug)]
pub struct Line(pub String);

/// A list of `sysctl.conf` paths.
#[derive(Default)]
pub struct SysCtlConfPaths(pub Vec<PathBuf>);
impl IntoIterator for SysCtlConfPaths {
    type Item = PathBuf;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<PathBuf> for SysCtlConfPaths {
    fn from_iter<T: IntoIterator<Item = PathBuf>>(iter: T) -> Self {
        SysCtlConfPaths(iter.into_iter().collect())
    }
}
