rm -rf artifacts
mkdir artifacts
cd artifacts

git clone https://github.com/had2020/limine-for-OpenContriOS.git --depth=1
cd limine-for-OpenContriOS
make
cd ..
mkdir -p iso/boot/limine
cd ..

cd kernel
sh build.sh
cd ..

cd artifacts

cp -r ../kernel/target/x86_64-unknown-none/debug/kernel iso/boot/kernel
cp limine-for-OpenContriOS/bin/limine-bios-cd.bin iso/boot/limine/
cp limine-for-OpenContriOS/bin/limine-uefi-cd.bin iso/boot/limine/
cp limine-for-OpenContriOS/limine.cfg iso/boot/limine/
