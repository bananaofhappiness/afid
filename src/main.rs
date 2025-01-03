use std::fs::File;
use std::path::PathBuf;
use clap::Parser;
use afid::compare_files_threaded_hash;
pub mod error;
use crate::error::Result;

#[derive(Parser)]
struct Cli {
    file1: PathBuf,
    file2: PathBuf,
    // TODO: add no_multhithreading flag
    // no_multithreading: bool,
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let file1 = File::open(args.file1)?;
    let file2 = File::open(args.file2)?;
    let res = compare_files_threaded_hash(file1, file2)?;
    match res {
        true => println!("Files are identical"),
        false => println!("Files are not identical"),
    }
    Ok(())
}
