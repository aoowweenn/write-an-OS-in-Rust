arch ?= x86_64
target ?= $(arch)-my_os
rust_os := target/$(target)/debug/libmy_os.a
build_dir ?= /dev/shm/build
kernel := $(build_dir)/kernel-$(arch).bin
iso := $(build_dir)/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	$(build_dir)/arch/$(arch)/%.o, $(assembly_source_files))

DEBUG ?= 1
ifeq ($(DEBUG), 1)
	cargoflag :=
else
	cargoflag := --release
endif

.PHONY: all clean run iso lint kernel

all: $(iso)

clean:
	@cargo clean
	@rm -r $(build_dir)

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

lint:
	@xargo clippy

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p $(build_dir)/isofiles/boot/grub
	@cp $(kernel) $(build_dir)/isofiles/boot/kernel.bin
	@cp $(grub_cfg) $(build_dir)/isofiles/boot/grub
	@grub-mkrescue -o $(iso) $(build_dir)/isofiles 2> /dev/null
	@rm -r $(build_dir)/isofiles

$(kernel): kernel $(assembly_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

kernel:
	@xargo build $(cargoflag)

# compile assembly files
$(build_dir)/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@yasm -felf64 $< -o $@
