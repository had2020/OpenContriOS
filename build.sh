# Only works on UNIX like OS. Tested on Arch Linux. 

rm -rf target
mkdir target

cd bootloader 
sh build.sh
cp boot ../target

cd ..

cd kernel
sh build.sh
cd target/debug
cp kernel ../../../target

cd ../../../

cd target
cat boot kernel >> build
