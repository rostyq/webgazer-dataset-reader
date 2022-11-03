use clap::Parser;
use std::path::PathBuf;

use webgazer_dataset_reader::{DatasetReader, WebcamVideoType};

#[derive(Parser)]
struct Args {
    path: PathBuf,
    video_type: String,
    index: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let video_type: WebcamVideoType = args.video_type.as_str().try_into().unwrap();

    let reader = DatasetReader::new(args.path);

    let readers = reader.participant_readers().unwrap();

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
