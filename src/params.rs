use std::str::FromStr;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Source {
    pub uri: Url,
    pub owner: String,
    pub token: String,
    pub package: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Version {
    pub version: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct GetStepParams {}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct CheckParams {
    pub source: Source,

    #[serde(default)]
    pub version: Option<Version>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct GetOutput<'a> {
    pub version: &'a Version,
}

impl<'a> From<&'a Version> for GetOutput<'a> {
    fn from(value: &'a Version) -> Self {
        Self { version: value }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct OutOutput<'a> {
    pub version: &'a Version,
}

impl<'a> From<&'a Version> for OutOutput<'a> {
    fn from(value: &'a Version) -> Self {
        Self { version: value }
    }
}

impl FromStr for CheckParams {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).context("Failed to deserialize check input")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct GetParams {
    pub source: Source,

    pub version: Version,
    // pub params: GetStepParams,
}

impl FromStr for GetParams {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).context("Failed to deserialize in input")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct OutStepParams {
    #[serde(default)]
    pub skip_if_exists: bool,
    pub version: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct OutParams {
    pub source: Source,
    pub params: OutStepParams,
}

impl FromStr for OutParams {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).context("Failed to deserialize out input")
    }
}
