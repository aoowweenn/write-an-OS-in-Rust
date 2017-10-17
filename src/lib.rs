#![feature(lang_items)]
#![no_std]

extern crate rlibc;

mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    let hello = b"Hello World";
    let color : u8 = 0x1F; // white fg, blue bg

    let mut hello_buffer = [color; 22];
    for (i, ch) in hello.into_iter().enumerate() {
        hello_buffer[i*2] = *ch;
    }

    let buffer_ptr = (0xB8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_buffer };
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
