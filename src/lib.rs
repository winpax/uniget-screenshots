pub mod v1;
pub mod v2;

pub use v2::*;

macro_rules! database_url {
    ($url:literal) => {
        pub const DATABASE_URL: &str = $url;
        pub fn database_url() -> url::Url {
            unsafe { url::Url::parse(DATABASE_URL).unwrap_unchecked() }
        }
    };
}

// The macro is not exported
#[allow(hidden_glob_reexports)]
pub(crate) use database_url;
