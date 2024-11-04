//! Data structures representing the v2 version of the database

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

super::database_url!(v2 "https://raw.githubusercontent.com/marticliment/UniGetUI/main/WebBasedData/screenshot-database-v2.json");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The root structure. Use this when parsing the JSON file.
pub struct Database {
    /// Information about the number of packages in the database.
    ///
    /// See [`PackageCount`] for more information.
    ///
    /// This is optional as not all databases have this information.
    /// However, the official database ([`DATABASE_URL`]), does have this information.
    pub package_count: Option<PackageCount>,
    /// A map of package names to their screenshots.
    ///
    /// Please note the package names are normalized.
    /// See [the documentation](https://marticliment.com/unigetui/help/icons-and-screenshots/#normalized-ids) for more information.
    pub icons_and_screenshots: HashMap<String, PackageImages>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A collection of information about the number of packages in the database.
pub struct PackageCount {
    /// The total number of packages in the database.
    pub total: u64,
    /// The number of packages that have an icon.
    ///
    /// Please note that this description may not be correct.
    /// Do not rely on this number being equal to [`PackageCount::packages_with_icon`].
    pub done: u64,
    /// The number of packages that have an icon.
    pub packages_with_icon: u64,
    /// The number of packages that have at least screenshot.
    pub packages_with_screenshot: u64,
    /// The total number of screenshots in the database.
    pub total_screenshots: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Holds the information about a package's icon and screenshots.
pub struct PackageImages {
    #[serde(deserialize_with = "deserialize_icon")]
    /// The URL to the package's icon.
    pub icon: Option<url::Url>,
    /// The URLs to the screenshots of the package.
    pub images: Vec<url::Url>,
}

fn deserialize_icon<'de, D>(deserializer: D) -> Result<Option<url::Url>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(url::Url::parse(&s).map_err(serde::de::Error::custom)?))
    }
}
