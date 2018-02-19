arch ?= x86_64
lld_link ?= lld-link
ovmf ?= /usr/share/ovmf/OVMF.fd

build_dir := target/$(arch)-pc-uefi/debug
efi_app := $(build_dir)/efi-test.efi
esp_image := $(build_dir)/esp.img
iso := $(build_dir)/efi_test.iso


.PHONY: all clean test


all: $(efi_app)


clean:
	@xargo clean


test: $(iso)
	@qemu-system-$(arch) -net none -bios $(ovmf) -cdrom $(iso)


$(efi_app): $(shell find src -type f)
	@RUST_TARGET_PATH=$(abspath ../targets) xargo build --target=$(arch)-pc-uefi


$(esp_image): $(efi_app)
	@mkdir -p $(build_dir)/esp/EFI/BOOT
	@cp $(efi_app) $(build_dir)/esp/EFI/BOOT/BOOTX64.EFI
	@rm -f $(esp_image)
	@dd if=/dev/zero of=$(esp_image) bs=1M count=64
	@mkfs.vfat -F 32 $(esp_image)
	@mcopy -i $(esp_image) -s $(build_dir)/esp/EFI ::


$(iso): $(esp_image)
	@mkdir -p $(build_dir)/iso
	@cp $(esp_image) $(build_dir)/iso/
	@xorriso -as mkisofs \
		-o $(iso) \
		-e $(notdir $(esp_image)) \
		-no-emul-boot \
		$(build_dir)/iso
