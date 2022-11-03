use std::path::PathBuf;
use clap::Parser;

use webgazer_dataset_reader::DatasetReader;

#[derive(Parser)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let reader = DatasetReader::new(args.path);

    for participant in reader.participant_characteristics().unwrap() {
        println!("{:#?}", participant);
    }
}