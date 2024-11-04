pub mod v2 {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    pub const DATABASE_URL: &str = "https://raw.githubusercontent.com/marticliment/UniGetUI/main/WebBasedData/screenshot-database-v2.json";

    pub fn database_url() -> url::Url {
        unsafe { url::Url::parse(DATABASE_URL).unwrap_unchecked() }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ScreenshotDatabase {
        package_count: PackageCount,
        packages: HashMap<String, PackageImages>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PackageImages {
        icon: url::Url,
        images: Vec<url::Url>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PackageCount {
        total: u64,
        done: u64,
        packages_with_icon: u64,
        packages_with_screenshot: u64,
        total_screenshots: u64,
    }
}

pub use v2::*;
