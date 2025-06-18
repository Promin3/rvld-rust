#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ElfHeader {
    // ident
    pub ident:      [u8; 16], // 16 bytes
    /// executable,shared object,relocatable etc.
    pub type_:      u16, // 2 bytes
    /// arm, x86, etc. 
    pub machine:    u16,
    /// version of the ELF format,typically 1 for original ELF
    pub version:    u32,
    /// entry point address, where execution starts
    pub entry:      u64,
    /// Points to the start of the program header table.
    pub phoff:      u64,
    /// Points to the start of the section header table.
    pub shoff:      u64,
    /// Interpretation of this field depends on the target architecture.
    pub flags:      u32,
    /// ELF header size
    pub ehsize:     u16,
    /// Contains the size of a program header table entry
    pub phentsize:  u16,
    /// Contains the number of entries in the program header table.
    pub phnum:      u16,
    /// Contains the size of a section header table entry.
    pub shentsize:  u16,
    /// Contains the number of entries in the section header table.
    pub shnum:      u16,
    /// Contains index of the section header table entry that contains the section names.
    pub shstrndx:   u16,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SectionHeader {
    /// An offset to a string in the .shstrtab section that represents the name of this section.
    pub name:       u32,
    /// program,string table,inactive etc. 
    pub type_:      u32,
    /// Writable, executable, allocatable etc.
    pub flags:      u64,
    /// Virtual address of the section in memory, for sections that are loaded.
    pub addr:       u64,
    /// Offset of the section in the file image.
    pub offset:     u64,
    /// Size in bytes of the section. May be 0.
    pub size:       u64,
    /// Contains the section index of an associated section.
    pub link:       u32,
    /// Contains extra information about the section.
    pub info:       u32,
    /// Contains the required alignment of the section. 
    pub addralign:  u64,
    /// Contains the size, in bytes, of each entry, for sections that contain fixed-size entries. Otherwise, this field contains zero.
    pub entsize:    u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Sym{
    pub name: u32,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
    pub val: u64,
    pub size: u64,
} 

pub const ELF_HDR_SIZE: usize = std::mem::size_of::<ElfHeader>();
pub const SECTION_HDR_SIZE: usize = std::mem::size_of::<SectionHeader>();
pub const SYM_SIZE: usize = std::mem::size_of::<Sym>();

pub fn elf_get_name(str_tab: &[u8], offset: u32) -> &str {
    let offset = offset as usize;
    let len = binary_search(&str_tab[offset..], 0).unwrap();
    return std::str::from_utf8(&str_tab[offset..(offset + len)]).unwrap();
}

fn binary_search(data: &[u8], sep: u8) -> Option<usize> {
    for i in 0..data.len() {
        if data[i] == sep {
            return Some(i);
        }
    }
    None
}