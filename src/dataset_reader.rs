use std::fs::File;
use std::io;
use std::path::PathBuf;

use crate::dataset_path::DatasetPath;
use crate::participant_characteristics::ParticipantCharacteristics;
use crate::ParticipantReader;

pub struct DatasetReader {
    pub path: DatasetPath,
}

impl DatasetReader {
    pub fn new(path: PathBuf) -> Self {
        Self { path: path.into() }
    }

    pub fn participant_characteristics(&self) -> io::Result<Vec<ParticipantCharacteristics>> {
        let p = self.path.participant_characteristics();
        let file = File::open(p)?;

        let mut reader = csv::Reader::from_reader(file);

        reader
            .deserialize()
            .map(|result| {
                let record: ParticipantCharacteristics = result?;
                Ok(record)
            })
            .collect()
    }

    pub fn participant_readers(&self) -> io::Result<Vec<ParticipantReader>> {
        Ok(self
            .participant_characteristics()?
            .into_iter()
            .map(|characteristics| ParticipantReader::new(&characteristics, self))
            .collect())
    }
}
