mod utils;
mod linker;
use linker::file::ElfFile;
use std::env::args;
use std::rc::Rc;
use utils::utils::assert;

use crate::linker::inputfile::InputFile;
fn main() {
    if args().len() < 2 {
        eprintln!("Usage: {} <input_file>", args().next().unwrap());
        std::process::exit(1);
    }

    let file = ElfFile::new(args().nth(1).unwrap());
    let inputfile = InputFile::new(Rc::new(file));

    // readelf -S out/tests/hello/a.o 
    assert(inputfile.elfsections.len() == 11);
}
