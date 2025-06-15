#[derive(Debug, Clone, Copy)]
pub struct ElfHeader {
    // ident
    pub ident:      [u8; 16],
    /// executable or shared object
    pub type_:      u16,
    /// arm, x86, etc. 
    pub machine:    u16,
    /// version of the ELF format,typically 1 for original ELF
    pub version:    u32,
    /// entry point address, where execution starts
    pub entry:      u64,
    /// program header table offset
    pub phoff:      u64,
    /// section header table offset
    pub shoff:      u64,
    /// flags, architecture-specific
    pub flags:      u32,
    /// ELF header size
    pub ehsize:     u16,
    /// size of a single program header entry
    pub phentsize:  u16,
    /// count of program headers
    pub phnum:      u16,
    /// size of a single section header entry
    pub shentsize:  u16,
    /// count of section headers
    pub shnum:      u16,
    /// index of the Name section in the section header table
    pub shstrndx:   u16,
}

#[derive(Debug, Clone, Copy)]
pub struct SectionHeader {
    /// section name 
    pub name:       u32,
    /// program,string table,inactive etc. 
    pub type_:      u32,
    /// allocated, executable etc.
    pub flags:      u64,
    /// section header address
    pub addr:       u64,
    /// section offset
    pub offset:     u64,
    /// size of the section
    pub size:       u64,
    pub link:       u32,
    pub info:       u32,
    pub addralign:  u64,
    pub entsize:    u64,
}

pub const ELF_HDR_SIZE: usize = std::mem::size_of::<ElfHeader>();
pub const SECTION_HDR_SIZE: usize = std::mem::size_of::<SectionHeader>();

