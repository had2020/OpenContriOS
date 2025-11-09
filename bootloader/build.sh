# Building only works for Arch Linux for now.
# You can search up the commands for you operating system if needed, for now.


#!/usr/bin/env bash
set -euo pipefail

VOLID="OpenContriOS"
ESP_LABEL="OPENCONTRI"

ISO="${VOLID}.iso"
ISOROOT="iso_root"
ESP_IMG="esp.img"
EFI_APP="target/x86_64-unknown-uefi/debug/bootloader.efi"

cargo build --target x86_64-unknown-uefi

truncate -s 16M "${ESP_IMG}"
mkfs.vfat -F32 -n "${ESP_LABEL}" "${ESP_IMG}"

mmd   -i "${ESP_IMG}" ::/EFI ::/EFI/BOOT
mcopy -i "${ESP_IMG}" "${EFI_APP}" ::/EFI/BOOT/BOOTX64.EFI

rm -rf "${ISOROOT}"
mkdir -p "${ISOROOT}/EFI/BOOT"
cp "${EFI_APP}" "${ISOROOT}/EFI/BOOT/BOOTX64.EFI"
cp "${ESP_IMG}" "${ISOROOT}/"

xorriso -as mkisofs \
  -R -J -V "${VOLID}" \
  -o "${ISO}" \
  -eltorito-alt-boot \
  -e "$(basename "${ESP_IMG}")" \
  -no-emul-boot \
  -isohybrid-gpt-basdat \
  -append_partition 2 0xef "${ESP_IMG}" \
  "${ISOROOT}"

echo "OK: wrote ${ISO}"
rm -rf "${ISOROOT}" "${ESP_IMG}"

