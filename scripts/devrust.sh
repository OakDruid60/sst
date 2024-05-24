#!/bin/bash
#devrust.sh
#cd "$HOME/storage/downloads/rustsrc"
#mc -b
#return 0

cm=$(awk  '/cm/ {print}' < jap.txt | awk -F = '{print $2}')
prj=$(awk '/prj/ {print}' < jap.txt | awk -F = '{print $2}')

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

#if [ $# -lt 2 ]; then
#    help
#    return 1
#fi

# get the project and command which are required
#prj=$1
#shift
#cm=$1
#shift

# intialize various other variables
# MYVAR=$(dialog --inputbox "THIS OUTPUT GOES TO FD 1" 5 25  --output-fd 1)
#exec 3>&1;
#result=$(dialog --inputbox test 0 0 2>&1 1>&3);
#exitcode=$?;
#exec 3>&-;
#echo $result $exitcode;

# ==================================================================
exec 3>&1
prj=$(dialog --no-cancel --no-tags --erase-on-exit --default-item $prj --no-hot-list --title "devrust.sh" \
	--menu "Select repository" 0 0 8 "sst" "sst development" "tui" "tui playground" 2>&1 1>&3)
exitcode=$?
if [ $exitcode == 255 ]; then
	exec 3>&-
	return $exitcode
fi

#
cm=$(dialog --no-cancel --no-tags --erase-on-exit --default-item $cm --no-hot-list --title "devrust.sh" --menu "Pick operation for $prj" 0 0 10 \
	"cd" "change dir" "run" "run" "check" "check" "fmt" "format source" "gendoc" "generate doc" "publish" "publish and zip source" 2>&1 1>&3)
exitcode=$?
exec 3>&-
if [ $exitcode == 255 ]; then
	return $exitcode
fi

echo  -e "prj="$prj"\ncm="$cm > ~/jap.txt

# dialog --clear

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

# dialog --clear

cd ~

if [[ $cm == 'cd' ]]; then
	# clr_cd
	echo -e "\n\033[36m Change directory\033[0m" $src $prj "\n"
	cd $src/$prj
	return 0

elif [[ $cm == 'fmt' ]]; then
	clr_cd
	cargo fmt
	cd ~

elif [[ $cm == 'ls' ]]; then
	clr_cd
	ls -al
	cd ~

elif [[ $cm == 'gendoc' || $cm == 'doc' ]]; then
	clr_cd
	echo -e "\033[36m Gendoc \033[0m " $prj "\n"
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

# return 0
