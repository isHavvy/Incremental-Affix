//! Game Log

use std::fmt::Display;

use bevy::{math::usize, prelude::*};
use itertools::Itertools;

#[derive(Debug)]
pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_message::<LogEntry>()
        ;
    }
}

#[derive(Debug, Message)]
pub struct LogEntry(pub String);

impl LogEntry {
    #[inline]
    pub fn new(into_string: impl Into<String>) -> Self {
        Self(into_string.into())
    }
}

impl From<String> for LogEntry {
    #[inline]
    fn from(value: String) -> Self {
        LogEntry(value)
    }
}

impl From<&str> for LogEntry {
    fn from(value: &str) -> Self {
        LogEntry(value.to_string())
    }
}

impl<IntoLogEntry: Into<LogEntry> + Display, const N: usize> From<[IntoLogEntry; N]> for LogEntry {
    fn from(value: [IntoLogEntry; N]) -> Self {
        LogEntry(value.iter().join(" "))
    }
}