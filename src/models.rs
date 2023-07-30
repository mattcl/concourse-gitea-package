use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Package {
    pub id: u64,
    pub version: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct PackageFile {
    pub name: String,
}
