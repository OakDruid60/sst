#!/bin/bash
#devrust.sh

# ===============================================================
# help
help() {
    echo
    echo "devrust [proj] [cmd] [args]"
    echo
}

# ===============================================================
# clr_cd
clr_cd() {
    clear
    cd $src/$prj
}

# ===============================================================
# gen_doc : assuming already in the base directory of the project
gen_doc() {
    echo -e "\n\033[36m Generating Documentation for \033[0m" $prj "\n"
    rm -rf $src/$prj/doc
    rm -rf $src/$prj/publish
    rm -rf $src/$prj/target
    rm -rf $src/doc
    sed -i '3s:.*:version = "'"${prjVers}"'-'"${dateDashTimeID}"'":' Cargo.toml
    cargo clean --target-dir "$HOME/rustbuild/${PWD##*/}"
    cargo build -r --target-dir "$HOME/rustbuild/${PWD##*/}"
    echo -e "\n\033[36m Creating doc \033[0m"
    cargo doc -q -r --no-deps --target-dir "$HOME/rustbuild/${PWD##*/}"
    cp -r ~/rustbuild/$prj/doc $src/$prj/doc
    rm -r ~/rustbuild/$prj/doc

}

# ===============================================================
# devrust.sh

if [ $# -lt 2 ]; then
    help
    return 1
fi

# get the project and command which are required
prj=$1
shift
cm=$1
shift

# intialize various other variables
src=~/storage/downloads/rustsrc
dateID=$(date +%y%m%d)
dateTimeID=$(date +%y%m%d_%H%M)
dateDashTimeID=$(date +%y%m%d-%H%M)

rustBuildDir=$HOME/rustbuild/
tDir=$rustBuildDir"${PWD##*/}"
zipFileName=$prj"_"$dateTimeID".zip"

prjVers="0.1.0"
if [ $prj == 'sst' ]; then
    prjVers="0.3.0"
fi    

# check if the project exists, if not it will be created
if [ -d $src/$prj ]; then
    echo
else
    echo "$prj not found in $src"
    if [ $cm == 'init' ]; then
        clear
        echo -e "\033[36m Creating "$prj" \033[37m in \033[0m" $src "\n"
        #        #echo "creating $prj"
        cd $src
        cargo init $prj
        cd $src/$prj
#        sed -i '3s:.*:version = "'"${prjVers}"'-'"${dateDashTimeID}"'":' Cargo.toml
        mkdir zipFiles
        mkdir scripts
        echo -e "#h1\n## h2\n### h3\nnormal\n**bold**\n" >README.md
        echo -e "/target\n/doc\n/scripts\n/zipFiles\n" >.gitignore
        cd $src/$prj
        gen_doc
#        mv $src/$prj/target/doc $src/$prj/doc
        rm -rf $src/$prj/target
        echo -e "\n\033[36m Listing for \033[0m" $PWD
        ll
        echo -e "\n\033[36m Initialization Complete\033[0m\n"
        cd ~
        return 0
    fi
    return 2
fi

cd ~

if [[ $cm == 'cd' ]]; then
    clr_cd

elif [[ $cm == 'fmt' ]]; then
    clr_cd
    cargo fmt
    cd ~

elif [[ $cm == 'ls' ]]; then
    clr_cd
    ls -al
    cd ~

elif [[ $cm == 'gendoc'  ||  $cm == 'doc' ]]; then
    clr_cd 
    echo -e "\033[36m Gendoc \033[0m "  $prj "\n"
    cargo fmt
    gen_doc
    cd ~
    echo -e "\033[36m Done \033[0m\n"

elif [[ $cm == 'publish' ]]; then
    clear
    rm -rf $src/$prj/publish
    rm -rf $src/doc

    clr_cd 
    echo -e "\033[36mPublishing "$prj" \033[37m" $PWD "\033[0m\n"

#    sed -i '3s\.*\version = "0.3.'"${dateID}"'"\' Cargo.toml

    cargo fmt

    gen_doc

    #
    echo -e "\n\033[36m Generating publish directory\033[0m" $src/$prj/publish
    cd ~
    mkdir $src/$prj/publish
    mkdir $src/$prj/publish/scripts

    cp -r $src/$prj/src $src/$prj/publish
    cp $src/$prj/Cargo.toml $src/$prj/publish
    cp $src/$prj/Cargo.lock $src/$prj/publish
    cp $src/$prj/README.md $src/$prj/publish
 #   cp -r ~/rustbuild/$prj/doc $src/$prj/publish/doc
 #   cp -r ~/rustbuild/$prj/doc $src/doc
    cp devrust.sh $src/$prj/scripts
    cp devrust.sh $src/$prj/publish/scripts

    echo -e "\n\033[36m Creating Zip\033[0m"

    #cd ~/storage/downloads/rustsrc/sst/publish
    #rm -f ~storage/downloads/rustsrc/sst.zip

    #
    cd $src/$prj/publish
    zipn=$src/$prj/zipFiles/$zipFileName
    echo $zipn
    zip -r -q $zipn *
    rm -r -f $src/$prj/publish
    echo -e "\n\033[36m DONE\033[0m"
    cd ~

else
    clear
    cd $src/$prj
    echo $*
    echo -e "\033[36m" $prj $cm "\033[37m" $PWD "\033[0m\n"
    cargo $cm --target-dir "$HOME/rustbuild/${PWD##*/}" -- $*

    cd ~
    echo -e "\n"
fi

return 0
