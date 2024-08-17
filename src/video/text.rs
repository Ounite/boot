use core::arch::asm;
use core::ascii;

pub fn print_char(c: ascii::Char) {
    unsafe {
        asm!(
        "int 0x10",
        in("ah") 0x0e_u8,
        in("al") c as u8,
        in("bh") 0x00_u8,
        in("bl") 0x07_u8,
        );
    };
}


pub fn print_string(s: &str) {
    unsafe { s.as_bytes().as_ascii_unchecked() }.iter().for_each(|c| print_char(*c));
}

pub fn print_newline() {
    print_char(ascii::Char::CarriageReturn);
    print_char(ascii::Char::LineFeed);
}

pub fn print_u16(n: u16) {
    let mut curr_coef = 10000;
    while curr_coef != 0 {
        print_char(unsafe { ascii::Char::digit_unchecked((n / curr_coef % 10) as u8) });
        curr_coef /= 10;
    };
}
