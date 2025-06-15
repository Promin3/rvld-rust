use std::mem;
#[allow(dead_code)]
pub fn fatal(msg: &str) -> ! {
    eprintln!("\x1b[0;1;31mfatal:\x1b[0m {}", msg);
    std::process::exit(1);
}

#[allow(dead_code)]
pub fn assert(condition: bool) {
    if !condition {
        fatal("assertion failed");
    }
}

#[allow(dead_code)]
pub fn read_to<T: Copy>(data: &[u8]) -> T {
    let size = mem::size_of::<T>();
    if data.len() < size {
        fatal(format!("Not enough data to read type: need {} bytes, got {}", size, data.len()).as_str());
    }
    unsafe {
        std::ptr::read(data.as_ptr() as *const T)
    }
}