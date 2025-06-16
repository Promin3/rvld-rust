mod linker;
use linker::file::ElfFile;
use std::env::args;
use std::rc::Rc;
use crate::linker::elf;
use crate::linker::objectfile::Objectfile;

fn main() {
    if args().len() < 2 {
        eprintln!("Usage: {} <input_file>", args().next().unwrap());
        std::process::exit(1);
    }

    let file = ElfFile::new(args().nth(1).unwrap());
    let mut objectfile = Objectfile::new(Rc::new(file));
    objectfile.parse();

    assert!(objectfile.inputfile.borrow().elf_section_hdrs.len() == 11);
    assert!(objectfile.inputfile.borrow().first_global.unwrap() == 10);
    assert!(objectfile.inputfile.borrow().elf_syms.len() == 12);

    objectfile.print_objectfile();

    for (_, sym) in objectfile.inputfile.borrow().elf_syms.iter().enumerate() {
        println!("{}", elf::elf_get_name(& objectfile.inputfile.borrow().symbol_strtab, sym.name));
    }
}
