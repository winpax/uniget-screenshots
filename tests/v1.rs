#![expect(deprecated)]

use uniget_screenshots::v1::*;

#[test]
fn test_download_database() {
    reqwest::blocking::get(DATABASE_URL).unwrap();
    reqwest::blocking::get(database_url()).unwrap();
}

#[test]
fn test_deserialize() {
    let _: Database = reqwest::blocking::get(DATABASE_URL)
        .unwrap()
        .json()
        .unwrap();
}
