use crate::{DatasetReader, ParticipantCharacteristic, ParticipantPath};

pub struct ParticipantReader {
    pub path: ParticipantPath,
}

impl ParticipantReader {
    pub fn new(
        characteristics: &ParticipantCharacteristic,
        dataset_reader: &DatasetReader,
    ) -> Self {
        Self {
            path: ParticipantPath::from(characteristics, dataset_reader),
        }
    }
}
