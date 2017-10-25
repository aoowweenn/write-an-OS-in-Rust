#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![feature(asm)]
#![no_std]

extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main(multiboot2_info_ptr: usize) {
    vga_buffer::clear_screen();
    println!("Hello World{}", {
        println!("no deadlock");
        "!"
    });

    let boot_info = unsafe { multiboot2::load(multiboot2_info_ptr) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("Memory areas:");
    memory_map_tag.memory_areas().for_each(|area| {
        println!("\tstart: {:#x}, length: {:#x}", area.base_addr, area.length);
    });

    panic!("Hi, panic");
}

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in file {} at {}", file, line);
    println!("\t{}", fmt);
    hlt();
    loop {}
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const fn hlt() {
    unsafe {
        asm!("HLT");
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const fn hlt() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
