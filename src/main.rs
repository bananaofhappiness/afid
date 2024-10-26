use std::fs::File;
use std::path::PathBuf;
use clap::Parser;
use afid::{compare_files_threaded_hash, compare_files_hash};
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
    let res;

    if file1.metadata().unwrap().len() <= 4096 && file2.metadata().unwrap().len() <= 4096{
        res = compare_files_hash(file1, file2)?;
    } else {
        res = compare_files_threaded_hash(file1, file2)?;
    }

    match res {
        true => println!("Files are identical"),
        false => println!("Files are not identical"),
    }
    Ok(())
}
