#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main() {
    vga_buffer::clear_screen();
    println!("Hello World{}", {
        println!("no deadlock");
        "!"
    });
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
