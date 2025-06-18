use crate::linker::elf_structures::{ELF_HDR_SIZE, SECTION_HDR_SIZE, SYM_SIZE, ElfHeader, Sym, SectionHeader};
use crate::linker::elf_file::ElfFile;
use crate::linker::magic::check_magic;
use crate::linker::utils::{fatal, read_to};
use std::rc::Rc;

pub struct InputFile{
    pub file: Rc<ElfFile>,
    pub elf_section_hdrs: Vec<SectionHeader>,
    pub elf_syms: Vec<Sym>,
    pub first_global: Option<i64>,
    pub sh_strtab: Vec<u8>,
    pub symbol_strtab: Vec<u8>,
}

impl InputFile {
    pub fn new(file: Rc<ElfFile>) -> Self {
        let mut inputfile = InputFile {
            file:Rc::clone(&file),
            elf_section_hdrs: Vec::new(),
            elf_syms: Vec::new(),
            sh_strtab: Vec::new(),
            first_global: None,
            symbol_strtab: Vec::new(),
        };

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

        inputfile.elf_section_hdrs.push(shdr.clone());
        
        contents = &contents[SECTION_HDR_SIZE..];
        
        for _ in 1..num_sections {
            inputfile.elf_section_hdrs.push(read_to::<SectionHeader>(contents));
            contents = &contents[SECTION_HDR_SIZE..];
        }

        let mut shstrndx = ehdr.shstrndx as usize;
        if ehdr.shstrndx == 0xffff {
            shstrndx = shdr.link as usize;
        }
        
        inputfile.sh_strtab = inputfile.get_bytes_from_id(shstrndx).to_vec();

        inputfile   
    }

    /// get section's bytes from section header 
    pub fn get_bytes_from_section_hdr(&self, section_header: &SectionHeader) -> &[u8] {
        let end = section_header.offset + section_header.size;
        let contents_len = self.file.contents.len();
        if end as usize > contents_len{
            fatal(format!("\x1b[0;1;31mDEBUG:\x1b[0m section header out of range: {} > {}",
            end, contents_len).as_str());
        }

        &self.file.contents[section_header.offset as usize..end as usize]
    }   

    /// get section's bytes from section header id
    pub fn get_bytes_from_id(&self, id: usize) ->&[u8]{
        self.get_bytes_from_section_hdr(&self.elf_section_hdrs[id])
    }

    /// find section header through it's type
    pub fn find_section_hdr_from_type(&self, ty: u32) -> Option<&SectionHeader> {
        for shdr in &self.elf_section_hdrs {
            if shdr.type_ == ty {
                return Some(shdr);
            }
        }
        None
    }

    /// fill up the elf_syms vector by reading symbols from a section header bytes sequence
    pub fn fillup_elf_syms(&mut self, s: Rc<SectionHeader>){
        let bs = self.get_bytes_from_section_hdr(&s).to_vec();
        let nums = bs.len() / SYM_SIZE;
        for i in 0..nums{
            let offset = i * SYM_SIZE;
            let sym = read_to::<Sym>(&bs[offset..]);
            self.elf_syms.push(sym);
        }
    }
}