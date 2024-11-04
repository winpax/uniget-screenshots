#![deprecated(note = "Use `v2` instead. This will not be removed for backwards compatibility.")]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

crate::database_url!("https://raw.githubusercontent.com/marticliment/UniGetUI/main/WebBasedData/screenshot-database.json");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenshotDatabase {
    package_count: PackageCount,
    winget: HashMap<String, PackageImages>,
    scoop: HashMap<String, PackageImages>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageCount {
    winget: u64,
    scoop: u64,
    total: u64,
    done: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageImages {
    icon: url::Url,
    images: Vec<url::Url>,
}