#!/bin/bash


#echo $HOME
#cd ~
#cd storage
#cd downloads
#cd rustsrc
#cd sst
rm -r -f  ~/storage/downloads/rustsrc/sst/publish
clear
cd ~/storage/downloads/rustsrc/sst
echo -e "\033[36mPublishing sst to android\033[37m"  $PWD "\033[0m\n"

cargo clean  --target-dir "$HOME/rustbuild/${PWD##*/}"
cargo build -r   --target-dir "$HOME/rustbuild/${PWD##*/}"
echo -e "\n\033[36mGenerating Documentaion" 
cargo doc -q -r --no-deps  --target-dir "$HOME/rustbuild/${PWD##*/}"

cd ~
echo -e "\nCopying to publish directory\033[0m"
mkdir ~/storage/downloads/rustsrc/sst/publish
cp -r ~/storage/downloads/rustsrc/sst/src ~/storage/downloads/rustsrc/sst/publish
cp ~/storage/downloads/rustsrc/sst/Cargo.toml ~/storage/downloads/rustsrc/sst/publish
cp ~/storage/downloads/rustsrc/sst/Cargo.lock ~/storage/downloads/rustsrc/sst/publish
cp ~/storage/downloads/rustsrc/sst/README.md  ~/storage/downloads/rustsrc/sst/publish
cp -r ~/rustbuild/sst/doc ~/storage/downloads/rustsrc/sst/publish/doc

echo -e "\n\033[36mCreating Zip\033[0m"

cd ~/storage/downloads/rustsrc/sst/publish
rm -f ~storage/downloads/rustsrc/sst.zip
zip -r -q ~/storage/downloads/rustsrc/sst.zip *
rm -r -f ~/storage/downloads/rustsrc/sst/publish
echo -e "\n\033[36mDONE\033[0m"
cd ~
