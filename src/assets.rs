use rust_embed::{Embed, EmbeddedFile};
use std::str;

#[derive(Embed)]
#[folder = "assets/"]
struct Asset;

pub fn open_assets(path: &str) -> Option<EmbeddedFile> {
    Asset::get(path)
}

pub fn read_to_string(path: &str) -> Option<String> {
    let raw = open_assets(path)?.data;
    Some(String::from_utf8(raw.into_owned()).unwrap())
}
