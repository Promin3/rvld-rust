pub fn check_magic(contents: &[u8]) -> bool {
    contents.starts_with(b"\x7fELF")
}
