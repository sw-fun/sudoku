//! Shared time formatting for the header and the win overlay.

/// Zero-padded `mm:ss` for elapsed seconds (minutes may exceed 99).
#[must_use]
pub fn mmss(secs: u32) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
}
