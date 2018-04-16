arch ?= x86_64
profile ?= debug
ovmf ?= /usr/share/ovmf/OVMF.fd

build_dir := target/$(arch)-pc-uefi/$(profile)
efi_app := $(build_dir)/test.efi
esp_image := $(build_dir)/esp.img
iso := $(build_dir)/efi_test.iso


.PHONY: all clean test


all: $(efi_app)


clean:
	@xargo clean


test: $(iso)
	@qemu-system-$(arch) -m 512 -net none -bios $(ovmf) -cdrom $(iso)


export RUST_TARGET_PATH=$(abspath targets)
ifeq ($(profile), debug)
	profile_arg :=
else
	profile_arg := --$(profile)
endif
$(efi_app): $(shell find src -type f)
	@xargo build \
		--target=$(arch)-pc-uefi \
		--bin test \
		$(profile_arg)


$(esp_image): $(efi_app)
	@mkdir -p $(build_dir)/esp/EFI/BOOT
	@cp $(efi_app) $(build_dir)/esp/EFI/BOOT/BOOTX64.EFI
	@echo "hello, world!" > $(build_dir)/esp/EFI/test.txt
	@rm -f $(esp_image)
	@dd if=/dev/zero of=$(esp_image) bs=1M count=64
	@mkfs.vfat -F 32 $(esp_image) -n EFISys
	@mcopy -i $(esp_image) -s $(build_dir)/esp/EFI ::


$(iso): $(esp_image)
	@mkdir -p $(build_dir)/iso
	@cp $(esp_image) $(build_dir)/iso/
	@xorriso -as mkisofs \
		-o $(iso) \
		-e $(notdir $(esp_image)) \
		-no-emul-boot \
		$(build_dir)/iso
