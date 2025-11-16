#!/bin/bash
set -e
rm -rf artifacts
mkdir artifacts
cd artifacts
git clone https://codeberg.org/Limine/Limine.git --branch=v10.x-binary --depth=1
mv Limine limine
make -C limine
mkdir -p iso_root
mkdir -p iso_root/boot
cd ..
cd kernel
sh build.sh
cd ..
cd artifacts
if [ "$1" = "x86" ]; then
    cp -v -r ../kernel/target/x86_64-unknown-none/debug iso_root/boot/
elif [ "$1" = "Risc-V" ]; then
    cp -v -r ../kernel/target/riscv64imac-unknown-none-elf/debug iso_root/boot/
elif [ "$1" = "Arm64" ]; then
    cp -v -r ../kernel/target/aarch64-unknown-none/debug iso_root/boot/
else
    echo 'Please use args x86, Risc-V, or Arm64'
    exit 1
fi

mkdir -p iso_root/boot/limine
cp -v limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/

mkdir -p iso_root/EFI/BOOT
cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/
cp -v limine/BOOTIA32.EFI iso_root/EFI/BOOT/
xorriso -as mkisofs -R -r -J -b boot/limine/limine-bios-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus \
        -apm-block-size 2048 --efi-boot boot/limine/limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        iso_root -o image.iso
./limine/limine bios-install image.iso
