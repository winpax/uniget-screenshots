use serde::{Deserialize, Serialize};
use std::collections::HashMap;

super::database_url!("https://raw.githubusercontent.com/marticliment/UniGetUI/main/WebBasedData/screenshot-database-v2.json");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[expect(deprecated)]
pub struct ScreenshotDatabase {
    package_count: PackageCount,
    packages: HashMap<String, super::v1::PackageImages>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageCount {
    total: u64,
    done: u64,
    packages_with_icon: u64,
    packages_with_screenshot: u64,
    total_screenshots: u64,
}
