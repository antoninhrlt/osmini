include build/build.mk

.SILENT : help
help :
	echo -e "\x1b[1m"
	echo -e "Help page for osmini" "\n"
	echo "Available commands (make) :"
	echo -e "\t" "build : build the project"
	echo -e "\t" "help : get this help page"
	echo -e "\t" "run : run the system into the qemu emulator"
	echo "Dependencies :"
	echo -e "\t" "(build) make, nasm, ld, "
	echo -e "\t" "(run) qemu-system-x86_64"
	echo -e "\x1b[0m"
