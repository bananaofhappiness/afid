use std::io::prelude::*;
use std::fs::File;
use std::sync::{Arc, Barrier, mpsc};
use std::thread;
pub mod error;
use crate::error::Result;

pub fn compare_files_threaded(mut file1: File, mut file2: File) -> Result<bool> {
    
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let barrier = Arc::new(Barrier::new(2));
    let barrier1 = Arc::clone(&barrier);
    let barrier2 = Arc::clone(&barrier);

    thread::spawn(move || {
        let mut buf = [0;128];
        'a: loop {
            let bytes_read = file1.read(&mut buf).unwrap();
            if bytes_read < buf.len() {
                for i in buf {
                    if let Err(_) = tx1.send(i16::from(i)) {
                        break 'a;
                    };
                    barrier1.wait();
                }
                if let Err(_) = tx1.send(-1) {
                    break 'a;
                };
                break;
            }
            for i in buf {
                if let Err(_) = tx1.send(i16::from(i)) {
                     break 'a;
                };
                barrier1.wait();
            } 
        }
    });

    thread::spawn(move || {
        let mut buf = [0;128];
        'a: loop {
            let bytes_read = file2.read(&mut buf).unwrap();
            if bytes_read < buf.len() {
                for i in buf {
                    if let Err(_) = tx2.send(i16::from(i)) {
                        break 'a;
                    };
                    barrier2.wait();
                }
                if let Err(_) = tx2.send(-1) {
                    break 'a;
                };
                break;
            }
            for i in buf {
                if let Err(_) = tx2.send(i16::from(i)) {
                     break 'a;
                };
                barrier2.wait();
            } 
        }
    });

    loop {
        let byte1 = rx1.recv().unwrap();
        let byte2 = rx2.recv().unwrap();

        if byte1 != byte2 {
            return Ok(false);
        }
        if byte1 == -1 && byte2 == -1 {
            return Ok(true);
        }
        
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn identical_files() -> Result<()> {
        let file1 = File::open("test_files/orig.txt")?;
        let file2 = File::open("test_files/orig copy.txt")?;
        let res = compare_files_threaded(file1, file2)?;
        assert_eq!(res, true);
        Ok(())
    }

    #[test]
    fn different_beggining() -> Result<()> {
        let file1 = File::open("test_files/orig.txt")?;
        let file2 = File::open("test_files/orig beginning.txt")?;
        let res = compare_files_threaded(file1, file2)?;
        assert_eq!(res, false);
        Ok(())
    }

    #[test]
    fn different_end() -> Result<()> {
        let file1 = File::open("test_files/orig.txt")?;
        let file2 = File::open("test_files/orig end.txt")?;
        let res = compare_files_threaded(file1, file2)?;
        assert_eq!(res, false);
        Ok(())
    }

    #[test]
    fn identical_photos() -> Result<()> {
        let file1 = File::open("test_files/banana.jpeg")?;
        let file2 = File::open("test_files/banana copy.jpeg")?;
        let res = compare_files_threaded(file1, file2)?;
        assert_eq!(res, true);
        Ok(())
    }

    #[test]
    fn one_pixel_difference() -> Result<()> {
        let file1 = File::open("test_files/banana.jpeg")?;
        let file2 = File::open("test_files/banana black pixel.jpeg")?;
        let res = compare_files_threaded(file1, file2)?;
        assert_eq!(res, false);
        Ok(())
    }

    #[test]
    fn different_pictures() -> Result<()> {
        let file1 = File::open("test_files/banana.jpeg")?;
        let file2 = File::open("test_files/no banana.jpeg")?;
        let res = compare_files_threaded(file1, file2)?;
        assert_eq!(res, false);
        Ok(())
    }
}