use std::{
    cmp::Ordering,
    fs::read_dir,
    io,
    path::{Path, PathBuf},
};

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

    pub fn webcam_videos(
        &self,
        video_type: Option<WebcamVideoType>,
        index: Option<usize>,
    ) -> io::Result<Vec<PathBuf>> {
        get_webcam_video_paths(&self.root, &self.participant_log_id, video_type, index)
    }

    pub fn webcam_video(
        &self,
        video_type: WebcamVideoType,
        index: usize,
        n: Option<usize>,
    ) -> io::Result<Option<PathBuf>> {
        self.webcam_videos(Some(video_type), Some(index))
            .map(|paths| paths.iter().nth(n.map_or(0, |n| n)).map(|p| p.to_owned()))
    }
}

pub fn get_webcam_video_paths<P: AsRef<Path>>(
    root: P,
    participant_log_id: &str,
    video_type: Option<WebcamVideoType>,
    index: Option<usize>,
) -> io::Result<Vec<PathBuf>> {
    let regex = Regex::new(&format!(
        r"{}_{}_-study-{}(\s\(\d+\))?\.webm",
        participant_log_id,
        index.map_or(r"\d+".to_string(), |i| i.to_string()),
        video_type.map_or(r"[a-z_]+", |t| t.as_str())
    ))
    .unwrap();

    // println!("{:?}", regex);

    let mut paths: Vec<PathBuf> = read_dir(root.as_ref().clone())?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        // .inspect(|path| println!("{:?}", path))
        .filter(|path| {
            path.file_name()
                .map(|name| name.to_str())
                .flatten()
                .map_or(false, |name| regex.is_match(name))
        })
        .collect();

    let regex_first_index = Regex::new(r"_(\d+)_").unwrap();
    let regex_aux_index = Regex::new(r"\((\d+)\)").unwrap();

    let parse_indices = |p: &PathBuf| {
        let name = p.file_name().map(|n| n.to_str()).flatten().unwrap();

        let index: usize = regex_first_index
            .captures(name)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let aux: Option<usize> = regex_aux_index
            .captures(name)
            .map(|caps| caps.get(1))
            .flatten()
            .map(|m| m.as_str())
            .map(|s| usize::from_str_radix(s, 10).ok())
            .flatten();

        (index, aux)
    };

    paths.sort_by(|a, b| -> Ordering {
        let (a_i, a_aux) = parse_indices(a);
        let (b_i, b_aux) = parse_indices(b);

        match a_i.cmp(&b_i) {
            Ordering::Equal => match (a_aux, b_aux) {
                (Some(a_), Some(b_)) => a_.cmp(&b_),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => Ordering::Equal,
            },
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    });

    Ok(paths)
}
