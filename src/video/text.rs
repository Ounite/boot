use core::arch::asm;

pub fn print_char(c: u8) {
    unsafe {
        asm!(
            "int 0x10",
            in("al") c,
            in("ah") 0x0e_u8,
        );
    };
}

pub fn print_string(s: &str) {
    s.as_bytes().iter().for_each(|c| print_char(*c));
}

pub fn print_newline() {
    print_char('\r' as u8);
    print_char('\n' as u8);
}
