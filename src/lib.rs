#![no_std]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![feature(asm)]
#![feature(inclusive_range)]
#![feature(range_contains)]

extern crate itertools;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use memory::FrameAllocator;

#[macro_use]
mod vga_buffer;

mod memory;

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

    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("kernel sections:");
    let kernel_boundary = elf_sections_tag
        .sections()
        .inspect(|section| {
            println!(
                "\taddr: {:#x}, size: {:#x}, flags: {:#x}",
                section.addr,
                section.size,
                section.flags
            );
        })
        .map(|s| (s.addr, s.addr + s.size))
        .minmax();

    let (kernel_start, kernel_end) = if let MinMax((s, _), (_, e)) = kernel_boundary {
        (s, e)
    } else {
        panic!("No valid kernel boundary!");
    };

    println!(
        "kernel_start: {:#x}, kernel_end: {:#x}",
        kernel_start,
        kernel_end
    );

    let multiboot_start = multiboot2_info_ptr;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    println!(
        "multiboot_start: {:#x}, multiboot_end: {:#x}",
        multiboot_start,
        multiboot_end
    );

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        multiboot_start,
        multiboot_end,
        memory_map_tag.memory_areas(),
    );

    for i in 0.. {
        if let None = frame_allocator.allocate() {
            println!("Total allocated frame: {}", i);
            break;
        }
    }

    panic!("Hi, panic");
}

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}
#[lang = "panic_fmt"]
#[no_mangle]
#[cfg_attr(feature = "cargo-clippy", allow(empty_loop))]
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
