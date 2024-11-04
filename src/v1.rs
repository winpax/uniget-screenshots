//! Data structures representing the v1 version of the database
//!
//! While this module is deprecated, it will not be removed for backwards compatibility.

#![deprecated(note = "If possible, v2 should be used instead")]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::v2::PackageImages;

crate::database_url!(v1 "https://raw.githubusercontent.com/marticliment/UniGetUI/main/WebBasedData/screenshot-database.json");

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
    /// A map of winget package names to their screenshots.
    ///
    /// Please note the package names are normalized.
    /// See [the documentation](https://marticliment.com/unigetui/help/icons-and-screenshots/#normalized-ids) for more information.
    pub winget: HashMap<String, PackageImages>,
    /// A map of Scoop package names to their screenshots.
    ///
    /// Please note the package names are normalized.
    /// See [the documentation](https://marticliment.com/unigetui/help/icons-and-screenshots/#normalized-ids) for more information.
    pub scoop: HashMap<String, PackageImages>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A collection of information about the number of packages in the database.
pub struct PackageCount {
    /// The total number of winget packages in the database.
    pub winget: u64,
    /// The total number of Scoop packages in the database.
    pub scoop: u64,
    /// The total number of packages in the database.
    pub total: u64,
    /// The number of packages that have an icon.
    ///
    /// Please note that this description may not be correct.
    /// Do not rely on it being correct.
    pub done: u64,
}
