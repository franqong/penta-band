use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SongProject {
    pub format_version: u32,
    pub title: String,
}

impl Default for SongProject {
    fn default() -> Self {
        Self {
            format_version: 1,
            title: String::new(),
        }
    }
}
