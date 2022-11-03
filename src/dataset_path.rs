use std::{path::PathBuf, convert::AsRef};

#[derive(Debug)]
pub struct DatasetPath(PathBuf);

impl DatasetPath {
    pub fn participant_characteristics(&self) -> PathBuf {
        self.0.join("participant_characteristics.csv")
    }
}

impl From<PathBuf> for DatasetPath {
    fn from(p: PathBuf) -> Self {
        Self(p)
    }
}

impl AsRef<PathBuf> for DatasetPath {
    fn as_ref(&self) -> &PathBuf {
        &self.0
    }
}