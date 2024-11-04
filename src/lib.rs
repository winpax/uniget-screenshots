#![doc = include_str!("../README.md")]
#![warn(
    clippy::all,
    clippy::pedantic,
    rust_2018_idioms,
    rustdoc::all,
    rust_2024_compatibility,
    missing_docs
)]
#![allow(clippy::module_name_repetitions)]

pub mod v1;
pub mod v2;

pub use v2::*;

macro_rules! database_url {
    (v1 $url:literal) => {
        /// The stringified URL to the official v1 database. Use [`database_url`] to get the URL as a [`url::Url`] struct.
        pub const DATABASE_URL: &str = $url;
        #[must_use]
        /// Get the official v1 URL as a [`url::Url`] struct.
        pub fn database_url() -> url::Url {
            unsafe { url::Url::parse(DATABASE_URL).unwrap_unchecked() }
        }
    };

    (v2 $url:literal) => {
        /// The stringified URL to the official v2 database. Use [`database_url`] to get the URL as a [`url::Url`] struct.
        pub const DATABASE_URL: &str = $url;
        #[must_use]
        /// Get the official v2 URL as a [`url::Url`] struct.
        pub fn database_url() -> url::Url {
            unsafe { url::Url::parse(DATABASE_URL).unwrap_unchecked() }
        }
    };
}

// The macro is not exported
#[allow(hidden_glob_reexports)]
pub(crate) use database_url;
