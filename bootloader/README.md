quick test

qemu-system-x86_64 \
  -machine q35 -m 256 \
  -drive if=pflash,format=raw,readonly=on,file=/usr/share/edk2/x64/OVMF_CODE.4m.fd \
  -drive if=pflash,format=raw,file=OVMF_VARS.fd \
  -drive file=uefi.img,if=virtio,format=raw \
  -serial stdio
