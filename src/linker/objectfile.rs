use crate::linker::config::SHT_SYMTAB;
use crate::linker::file::ElfFile;
use crate::linker::{elf::SectionHeader, inputfile::InputFile};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Objectfile{
    pub inputfile: Rc<RefCell<InputFile>>, 
    pub symtab_sec_hdr: Option<Rc<SectionHeader>>,
}

impl Objectfile {
    pub fn new(file: Rc<ElfFile>) -> Objectfile{
        Objectfile {
             inputfile: Rc::new(RefCell::new(InputFile::new(file))),
              symtab_sec_hdr: None,
            }
    }

    pub fn parse(&mut self){
        // get symbol table section header by type SHT_SYMTAB
        self.symtab_sec_hdr = self.inputfile
                        .borrow()
                        .find_section_hdr_from_type(SHT_SYMTAB)
                        .map(|sec| Rc::new(sec.clone()));
        
        if let Some(ref sym_sec_hdr) = self.symtab_sec_hdr {
            let mut inputfile = self.inputfile.borrow_mut();
            // info field in symbol section header is the index of the first global symbol
            inputfile.first_global = Some(sym_sec_hdr.info as i64);
            inputfile.fillup_elf_syms(sym_sec_hdr.clone());
            // link field in symbol section header is the index of the symbol name str table section
            // println!("link field in symbol section header: {}", sym_sec_hdr.link); == 9
            inputfile.symbol_strtab = inputfile
                                    .get_bytes_from_id(sym_sec_hdr.link as usize)
                                    .to_vec();

        }   
    }

    pub fn print_objectfile(&self) {
        let inputfile = self.inputfile.borrow();
        println!("{:#?}\n {:#?}\n {:#?}\n {:#?}\n",inputfile.elf_section_hdrs, inputfile.first_global ,inputfile.elf_syms, self.symtab_sec_hdr);
    }
}