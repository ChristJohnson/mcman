use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::utils::hashing::HashFormat;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MRPackFile {
    path: String,
    hashes: HashMap<HashFormat, String>,
    env: Option<Env>,
    file_size: u64,
    downloads: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Env {
    pub client: EnvSupport,
    pub server: EnvSupport,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EnvSupport {
    Required,
    Optional,
    Unsupported,
}
