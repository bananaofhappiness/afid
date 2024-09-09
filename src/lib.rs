use std::io::prelude::*;
use std::fs::File;
pub mod error;
use crate::error::Result;

pub fn compare_files(mut file1: &File, mut file2: &File) -> Result<bool> {
    let mut buf1 = [0;64];
    let mut buf2 = [0;64];
    loop {
        let bytes_read1 = file1.read(&mut buf1)?;
        let bytes_read2 = file2.read(&mut buf2)?;
        if bytes_read1 == 0 && bytes_read2 == 0 {
            return Ok(true);
        }
        if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
            return Ok(false);
        };
    }
}

pub mod bench {
    use super::*;

    pub fn compare_files_128k(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;131072];
        let mut buf2 = [0;131072];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }
    
    pub fn compare_files_64k(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;65536];
        let mut buf2 = [0;65536];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }
    
    pub fn compare_files_32k(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;32768];
        let mut buf2 = [0;32768];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }
    
    pub fn compare_files_16k(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;16384];
        let mut buf2 = [0;16384];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }
    
    pub fn compare_files_8k(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;8192];
        let mut buf2 = [0;8192];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }
    
    pub fn compare_files_4k(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;4096];
        let mut buf2 = [0;4096];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }

    pub fn compare_files_2k(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;2048];
        let mut buf2 = [0;2048];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }
    
    pub fn compare_files_1k(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;1024];
        let mut buf2 = [0;1024];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }

    pub fn compare_files_512(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;512];
        let mut buf2 = [0;512];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }
    
    pub fn compare_files_256(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;256];
        let mut buf2 = [0;256];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }
    
    pub fn compare_files_128(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;128];
        let mut buf2 = [0;128];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }

    pub fn compare_files_64(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;64];
        let mut buf2 = [0;64];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }

    pub fn compare_files_32(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;32];
        let mut buf2 = [0;32];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }

    pub fn compare_files_16(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;16];
        let mut buf2 = [0;16];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }

    pub fn compare_files_8(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;8];
        let mut buf2 = [0;8];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }

    pub fn compare_files_4(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;4];
        let mut buf2 = [0;4];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }

    pub fn compare_files_2(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;2];
        let mut buf2 = [0;2];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }

    pub fn compare_files_1(mut file1: &File, mut file2: &File) -> Result<bool> {
        let mut buf1 = [0;1];
        let mut buf2 = [0;1];
        loop {
            let bytes_read1 = file1.read(&mut buf1)?;
            let bytes_read2 = file2.read(&mut buf2)?;
            if bytes_read1 == 0 && bytes_read2 == 0 {
                return Ok(true);
            }
            if bytes_read1 != bytes_read2 || buf1[..bytes_read1] != buf2[..bytes_read2] {
                return Ok(false);
            };
        }
    }
}