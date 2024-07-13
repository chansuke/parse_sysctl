//! Defining the types for the application.
use std::collections::HashMap;

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

#[derive(Debug)]
pub struct Line(pub String);
