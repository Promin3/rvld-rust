use crate::linker::utils::fatal;
use std::fs::File;
use std::io::Read;

pub struct ElfFile {
    pub name: String,
    pub contents:  &'static [u8],
}

impl ElfFile {
    pub fn new(filename: String) -> Self{

        
        let mut f = File::open(&filename).unwrap_or_else(|_| {
            fatal(&format!("Failed to read file: {}", filename).as_str());
        });

        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        let contents= Box::new(buffer);

        // 用 box leak将运行时变量转为 static 全局生命周期的变量
        ElfFile { 
            name: filename, 
            contents:Box::leak(contents),
        }
    }
}