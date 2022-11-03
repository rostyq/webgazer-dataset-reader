use crate::{DatasetReader, ParticipantCharacteristics, ParticipantPath};

pub struct ParticipantReader {
    pub path: ParticipantPath,
}

impl ParticipantReader {
    pub fn new(
        characteristics: &ParticipantCharacteristics,
        dataset_reader: &DatasetReader,
    ) -> Self {
        Self {
            path: ParticipantPath::new(characteristics, dataset_reader),
        }
    }
}
