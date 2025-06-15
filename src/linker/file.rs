use utils::utils::fatal;
use std::fs::File;
use crate::utils;
use std::io::Read;

pub struct ElfFile {
    pub name: String,
    pub contents: Vec<u8>,
}

impl ElfFile {
    pub fn new(filename: String) -> Self{
        // println!("\x1b[0;1;31mDEBUG:\x1b[0m Trying to read file: {}", filename);
        // println!("\x1b[0;1;31mDEBUG:\x1b[0m Current working directory: {:?}", std::env::current_dir().unwrap());
        // println!("\x1b[0;1;31mDEBUG:\x1b[0m File exists: {}", std::path::Path::new(&filename).exists());

        let mut f = File::open(&filename).unwrap_or_else(|_| {
            fatal(&format!("Failed to read file: {}", filename).as_str());
        });

        let mut contents = Vec::new();
        f.read_to_end(&mut contents).unwrap();
        ElfFile { name: filename, contents }
    }
}