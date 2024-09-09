build:
	cargo bootimage

run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86-64-nyjako_os/debug/bootimage-nyjako_os.bin