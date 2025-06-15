use crate::linker::elf::{ELF_HDR_SIZE, SECTION_HDR_SIZE, ElfHeader, SectionHeader};
use crate::linker::file::ElfFile;
use crate::linker::magic::check_magic;
use crate::utils::utils::{fatal, read_to};
use std::rc::Rc;

pub struct InputFile{
    pub file: Rc<ElfFile>,
    pub elfsections: Vec<SectionHeader>,
}

impl InputFile {
    pub fn new(file: Rc<ElfFile>) -> Self {
        if file.contents.len() < ELF_HDR_SIZE {
            fatal("File is too small to be a valid ELF file");
        }

        if !check_magic(&file.contents) {
            fatal("File is not a valid ELF file");
        }

        let ehdr = read_to::<ElfHeader>(&file.contents);
        // 使用切片引用，避免拷贝
        let mut contents = &file.contents[ehdr.shoff as usize..];
        
        if contents.len() < SECTION_HDR_SIZE {
            fatal("File too small for section headers");
        }
        
        let shdr = read_to::<SectionHeader>(contents);
        let mut num_sections = ehdr.shnum as u64;

        // 当一个ELF文件有超过65535个section时，ELF规范定义了一种扩展机制
        // ehdr.ShNum被设置为0（表示使用扩展机制）
        // 实际的section数量存储在第一个section header的Size字段中
        if num_sections == 0 {
            num_sections = shdr.size;
        }

        let mut elfsections = Vec::with_capacity(num_sections as usize);
        elfsections.push(shdr);
        
        contents = &contents[SECTION_HDR_SIZE..];
        
        for _ in 1..num_sections {
            elfsections.push(read_to::<SectionHeader>(contents));
            contents = &contents[SECTION_HDR_SIZE..];
        }
        
        InputFile {
            file,
            elfsections,
        }
    }
}