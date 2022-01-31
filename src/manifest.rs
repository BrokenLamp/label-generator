use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub root: String,
    pub prefix: String,
    pub folders: Vec<String>,
}
