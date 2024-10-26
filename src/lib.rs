use std::io::prelude::*;
use std::fs::File;
use std::sync::{Arc, Barrier, mpsc};
use std::thread;
use xxhash_rust::xxh3::xxh3_64;
pub mod error;
use crate::error::Result;

const BUF_SIZE: usize = 1024;

pub fn compare_files_threaded_hash(mut file1: File, mut file2: File) -> Result<bool> {
    
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let barrier = Arc::new(Barrier::new(2));
    let barrier1 = Arc::clone(&barrier);
    let barrier2 = Arc::clone(&barrier);

    thread::spawn(move || {
        let mut buf = [0;BUF_SIZE];
        'a: loop {
            let bytes_read = file1.read(&mut buf).unwrap();
            if bytes_read < buf.len() {
                let hash = xxh3_64(&buf[..bytes_read]);
                if let Err(_) = tx1.send(hash) {
                    break 'a;
                }
                break 'a;
            }
            let hash = xxh3_64(&buf);
            if let Err(_) = tx1.send(hash) {
                break 'a;
            }
            barrier1.wait();
        }
    });

    thread::spawn(move || {
        let mut buf = [0;BUF_SIZE];
        'a: loop {
            let bytes_read = file2.read(&mut buf).unwrap();
            if bytes_read < buf.len() {
                let hash = xxh3_64(&buf[..bytes_read]);
                if let Err(_) = tx2.send(hash) {
                    break 'a;
                }
                break 'a;
            }
            let hash = xxh3_64(&buf);
            if let Err(_) = tx2.send(hash) {
                break 'a;
            }
            barrier2.wait();
        }
    });

    loop {
        let hash1 = rx1.recv();
        let hash2 = rx2.recv();

        if hash1 != hash2 {
            return Ok(false);
        }
        if hash1.is_err() && hash2.is_err() {
            return Ok(true);
        }
    }
}

pub fn compare_files_hash(mut file1: File, mut file2: File) -> Result<bool> {
    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();

    file1.read_to_end(&mut buf1)?;
    file2.read_to_end(&mut buf2)?;

    let hash1 = xxh3_64(&buf1);
    let hash2 = xxh3_64(&buf2);

    if hash1 == hash2 {
        return Ok(true);
    }

    Ok(false)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn identical_files() -> Result<()> {
        let file1 = File::open("test_files/orig.txt")?;
        let file2 = File::open("test_files/orig_copy.txt")?;
        let res = compare_files_threaded_hash(file1, file2)?;
        assert_eq!(res, true);
        Ok(())
    }

    #[test]
    fn different_beggining() -> Result<()> {
        let file1 = File::open("test_files/orig.txt")?;
        let file2 = File::open("test_files/orig_beginning.txt")?;
        let res = compare_files_threaded_hash(file1, file2)?;
        assert_eq!(res, false);
        Ok(())
    }

    #[test]
    fn different_end() -> Result<()> {
        let file1 = File::open("test_files/orig.txt")?;
        let file2 = File::open("test_files/orig_end.txt")?;
        let res = compare_files_threaded_hash(file1, file2)?;
        assert_eq!(res, false);
        Ok(())
    }

    #[test]
    fn identical_photos() -> Result<()> {
        let file1 = File::open("test_files/banana.jpeg")?;
        let file2 = File::open("test_files/banana_copy.jpeg")?;
        let res = compare_files_threaded_hash(file1, file2)?;
        assert_eq!(res, true);
        Ok(())
    }

    #[test]
    fn one_pixel_difference() -> Result<()> {
        let file1 = File::open("test_files/banana.jpeg")?;
        let file2 = File::open("test_files/banana_black_pixel.jpeg")?;
        let res = compare_files_threaded_hash(file1, file2)?;
        assert_eq!(res, false);
        Ok(())
    }

    #[test]
    fn different_pictures() -> Result<()> {
        let file1 = File::open("test_files/banana.jpeg")?;
        let file2 = File::open("test_files/no_banana.jpeg")?;
        let res = compare_files_threaded_hash(file1, file2)?;
        assert_eq!(res, false);
        Ok(())
    }
}
