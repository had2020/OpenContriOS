# Only tested on Arch Linux (WIP) build shell script.

cargo build --target x86_64-unknown-uefi

truncate -s 64M uefi.img
mkfs.vfat -n UEFI uefi.img
mmd   -i uefi.img ::/EFI ::/EFI/BOOT
mcopy -i uefi.img target/x86_64-unknown-uefi/debug/bootloader.efi ::/EFI/BOOT/BOOTX64.EFI
