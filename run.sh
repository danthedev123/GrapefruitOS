set -x

cargo build

# Install limine
git clone https://github.com/limine-bootloader/limine.git --branch=v3.0-branch-binary --depth=1
make -C limine

rm -rf iso_build
mkdir -p iso_build
cp target/x86_64-grapefruit/debug/grapefruit_os limine.cfg limine/limine.sys limine/limine-cd.bin limine/limine-cd-efi.bin iso_build/
xorriso -as mkisofs -b limine-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot limine-cd-efi.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    iso_build -o grapefruit.iso
limine/limine-deploy grapefruit.iso
rm -rf iso_build

qemu-system-x86_64 -cdrom grapefruit.iso --no-reboot --no-shutdown -d int -D kernel.log -m 256M