use crate::linker::machine_type::{MachineType, MACHINE_TYPE_NONE};

pub struct ContextArgs{
    pub output: String,
    pub emulation: MachineType,
    pub library_paths: Vec<String>,
}

pub struct Context {
    pub args: ContextArgs,
}

impl Context{
    pub fn new() -> Self{
        Context{
            args: ContextArgs {
                output: "a.out".to_string(), 
                emulation: MACHINE_TYPE_NONE, 
                library_paths: Vec::new(),
            }
        }
    }
}