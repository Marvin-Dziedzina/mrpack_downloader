use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct MrPack {
    pub format_version: i32,
    pub game: String,
    pub version_id: String,
    pub name: String,
    pub files: Vec<File>,
    pub dependencies: Dependencies,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct File {
    pub path: PathBuf,
    pub hashes: Hashes,
    pub env: Env,
    pub downloads: Vec<String>,
    pub file_size: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Hashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Env {
    pub client: Requirement,
    pub server: Requirement,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Requirement {
    Required,
    Unsupported,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Dependencies {
    pub forge: String,
    pub minecraft: String,
}
