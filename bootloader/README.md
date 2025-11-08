Resources: 
- https://os.phil-opp.com/freestanding-rust-binary/
- http://littleosbook.github.io/

# Building Bootloader

1.) Add the compulation target to rustup.
``` bash 
rustup target add x86_64-unknown-uefi
```

2.) Building the efi file from cargo.
``` bash
cargo build --target x86_64-unknown-uefi
```

3.) Creating the Img of the bootloader (Command for Arch Linux, other operating systems may vary!)
``` bash
dd if=/dev/zero of=uefi.img bs=1M count=64
mkfs.vfat uefi.img   # (aka mkfs.fat on some systems)
mmd   -i uefi.img ::/EFI ::/EFI/Boot
mcopy -i uefi.img target/x86_64-unknown-uefi/debug/bootloader.efi ::/EFI/Boot/BOOTX64.EFI
```

# Testing on QEMU 

1.) Install UEFI BIOS for QEMU

Arch 
``` bash 
sudo pacman -S edk2-ovmf qemu
```

2.) Run QEMU with the matching 3MB firmware image 
``` bash
qemu-system-x86_64 \
  -machine q35 \
  -cpu max \
  -m 512 \
  -serial stdio \
  -display gtk \
  -drive if=pflash,format=raw,unit=0,readonly=on,file=/usr/share/edk2/x64/OVMF_CODE.4m.fd \
  -drive if=pflash,format=raw,unit=1,file=./OVMF_VARS.fd \
  -drive file=uefi.img,format=raw,if=ide
```
