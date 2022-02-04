use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub root: String,
    pub sku: String,
    #[serde(default)]
    pub ignore: Vec<String>,
}
