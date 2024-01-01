#!/bin/bash

#echo $HOME
#cd ~
#cd storage
#cd downloads
#cd rustsrc
#cd sst
clear
cd ~/storage/downloads/rustsrc/sst
echo -e "\033[36m"  $1 "\033[37m"  $PWD "\033[0m\n"
cargo $1  --target-dir "$HOME/rustbuild/${PWD##*/}"

cd ~
echo -e "\n"
