use std::fs::File;
use std::path::PathBuf;
use clap::Parser;
use afid::compare_files;
pub mod error;
use crate::error::Result;

#[derive(Parser)]
struct Cli {
    file1: PathBuf,
    file2: PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let file1 = File::open(args.file1)?;
    let file2 = File::open(args.file2)?;
    let res = compare_files(&file1, &file2)?;
    match res {
        true => println!("Files are identical"),
        false => println!("Files are not identical"),
    }
    Ok(())
}