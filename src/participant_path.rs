use std::path::PathBuf;

use crate::{DatasetReader, ParticipantCharacteristics};

#[derive(Debug)]
pub struct ParticipantPath {
    pub participant_id: String,
    pub participant_log_id: String,
    pub root: PathBuf,
}

impl ParticipantPath {
    pub fn new(
        characteristics: &ParticipantCharacteristics,
        dataset_reader: &DatasetReader,
    ) -> Self {
        let root = dataset_reader.path.participant(characteristics);
        Self {
            participant_id: characteristics.participant_id.clone(),
            participant_log_id: characteristics.participant_log_id.as_millis().to_string(),
            root,
        }
    }

    pub fn specs(&self) -> PathBuf {
        self.root.join("specs.txt")
    }

    pub fn screen_recording(&self) -> PathBuf {
        self.root
            .join(self.participant_id.clone())
            .with_extension("mov")
    }

    pub fn user_interaction_logs(&self) -> PathBuf {
        self.root
            .join(self.participant_log_id.clone())
            .with_extension("json")
    }

    pub fn gaze_predictions(&self) -> PathBuf {
        self.root
            .join(self.participant_id.clone())
            .with_extension("txt")
    }
}
