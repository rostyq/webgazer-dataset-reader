use clap::Parser;
use std::path::PathBuf;

use webgazer_dataset_reader::{DatasetReader, WebcamVideoType};

#[derive(Parser)]
struct Args {
    path: PathBuf,
    video_type: Option<String>,
    index: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let video_type: Option<WebcamVideoType> = args.video_type.map(|t| t.as_str().try_into().unwrap());

    let reader = DatasetReader::new(args.path);

    let mut readers = reader.participant_readers().unwrap();

    let parse_id = |s: String| {
        usize::from_str_radix(s.split("_").nth(1).unwrap(), 10).unwrap()
    };

    readers.sort_by(|a, b| {
        let id1 = parse_id(a.path.participant_id().to_owned());
        let id2 = parse_id(b.path.participant_id().to_owned());

        id1.cmp(&id2)
    });

    for preader in readers {
        println!("{}", preader.path.participant_id());

        let video_paths = preader
            .path
            .webcam_videos(video_type, args.index)
            .unwrap();

        if video_paths.is_empty() {
            println!("\tNo videos found.");
        }

        for video_path in video_paths {
            println!("\t{}", video_path.file_name().map(|n| n.to_str()).flatten().unwrap());
        }
    }
}
