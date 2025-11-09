# Building only works for Arch Linux for now.
# You can search up the commands for you operating system if needed, for now.

cargo build --target x86_64-unknown-uefi 

dd if=/dev/zero of=efi.img bs=1M count=64
mkfs.vfat efi.img   # (aka mkfs.fat on some systems)
mmd   -i efi.img ::/EFI ::/EFI/Boot
mcopy -i efi.img target/x86_64-unknown-uefi/debug/bootloader.efi ::/EFI/Boot/BOOTX64.EFI

mkdir -p iso_root
cp efi.img iso_root/

xorriso -as mkisofs \
  -R -J -V UEFI_APP \
  -eltorito-platform efi \
  -eltorito-boot efi.img \
  -no-emul-boot \
  -isohybrid-gpt-basdat \
  -o OpenContriOS.iso \
  iso_root

rm -rf iso_root
rm efi.img
