use std::path::PathBuf;

use crate::ParticipantCharacteristics;

#[derive(Debug)]
pub struct DatasetPath(PathBuf);

impl DatasetPath {
    pub fn participant_characteristics(&self) -> PathBuf {
        self.0.join("participant_characteristics.csv")
    }

    pub fn participant(&self, chars: &ParticipantCharacteristics) -> PathBuf {
        self.0.join(chars.participant_id.clone())
    }
}

impl From<PathBuf> for DatasetPath {
    fn from(p: PathBuf) -> Self {
        Self(p)
    }
}
