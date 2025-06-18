use crate::linker::{file_type::{get_file_type, FILE_TYPE_OBJECT}, utils::read_to};

pub type MachineType = u8;

pub const MACHINE_TYPE_NONE: MachineType = 0;
pub const MACHINE_TYPE_RV64: MachineType = 1;


pub const EM_RISCV:u16 = 243;
pub const ELFCLASS64:u8 = 2;


pub fn get_machinetype_from_contents(contents: &[u8]) -> MachineType{
    let filetype = get_file_type(&contents);

    match filetype {
        FILE_TYPE_OBJECT =>{
            let machine = read_to::<u16>(&contents[18..]);
            if machine == EM_RISCV{
                 let class = &contents[4];
                 match class {
                    &ELFCLASS64 => return MACHINE_TYPE_RV64,
                    _ => {}
                 }
            }
        },
        _ => {},
    }

    return MACHINE_TYPE_NONE
}

pub struct MachineTypeStringer {
    pub machine_type: MachineType,
}


impl MachineTypeStringer {
    pub fn string(&self) -> String{
        match self.machine_type{
            MACHINE_TYPE_RV64 => {
                return "riscv64".to_string();
            },
            _ => {},
        }
        return "none".to_string();
    }
}
