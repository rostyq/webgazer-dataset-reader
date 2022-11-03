use std::{fs::read_dir, io, path::PathBuf};

use regex::Regex;

use crate::{DatasetReader, ParticipantCharacteristic, ParticipantLogId, WebcamVideoType};

#[derive(Debug)]
pub struct ParticipantPath {
    pub participant_log_id: ParticipantLogId,
    pub root: PathBuf,
}

impl ParticipantPath {
    pub fn from(
        characteristics: &ParticipantCharacteristic,
        dataset_reader: &DatasetReader,
    ) -> Self {
        let root = dataset_reader
            .path
            .as_ref()
            .join(characteristics.participant_id.clone());
        Self {
            participant_log_id: characteristics.participant_log_id.clone(),
            root,
        }
    }

    pub fn participant_id<'a>(&'a self) -> &'a str {
        self.root.file_name().map(|n| n.to_str()).flatten().unwrap()
    }

    pub fn specs(&self) -> PathBuf {
        self.root.join("specs.txt")
    }

    pub fn screen_recording(&self) -> PathBuf {
        self.root
            .join(self.participant_id().clone())
            .with_extension("mov")
    }

    pub fn user_interaction_logs(&self) -> PathBuf {
        self.root
            .join(self.participant_log_id.clone())
            .with_extension("json")
    }

    pub fn gaze_predictions(&self) -> PathBuf {
        self.root
            .join(self.participant_id().clone())
            .with_extension("txt")
    }

    pub fn webcam_videos(&self, video_type: WebcamVideoType) -> io::Result<Vec<PathBuf>> {
        let regex = Regex::new(&format!(
            r"{}_\d+_-study-{}(\s\(\d+\))?\.webm",
            self.participant_id(),
            video_type.as_str()
        ))
        .unwrap();

        let mut paths: Vec<PathBuf> = read_dir(self.root.clone())?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .filter(|path| {
                path.file_name()
                    .map(|name| name.to_str())
                    .flatten()
                    .map_or(false, |name| regex.is_match(name))
            })
            .collect();

        paths.sort();

        Ok(paths)
    }
}
