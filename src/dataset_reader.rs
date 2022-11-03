use std::io;
use std::path::PathBuf;

use crate::dataset_path::DatasetPath;
use crate::participant_characteristics::ParticipantCharacteristic;
use crate::{ParticipantReader, load_participant_characteristics};

pub struct DatasetReader {
    pub path: DatasetPath,
}

impl DatasetReader {
    pub fn new(path: PathBuf) -> Self {
        Self { path: path.into() }
    }

    pub fn participant_characteristics(&self) -> io::Result<Vec<ParticipantCharacteristic>> {
        load_participant_characteristics(self.path.participant_characteristics())
    }

    pub fn participant_readers(&self) -> io::Result<Vec<ParticipantReader>> {
        Ok(self
            .participant_characteristics()?
            .into_iter()
            .map(|characteristics| ParticipantReader::new(&characteristics, self))
            .collect())
    }
}
