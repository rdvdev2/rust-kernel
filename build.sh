#!/usr/bin/env sh
set -e

PROFILE=${1:-debug}
TARGET=i686-unknown-none
TARGET_DIR=target/$TARGET/$PROFILE
SYSROOT=$TARGET_DIR/sysroot

if [ $PROFILE != "debug" ]; then
	cargo build --profile $PROFILE
else
	cargo build
fi

grub-file --is-x86-multiboot2 $TARGET_DIR/rust-kernel
mkdir -p $SYSROOT
cp $TARGET_DIR/rust-kernel $SYSROOT/kernel

cp -rv sysroot/* $SYSROOT

grub-mkrescue -o $TARGET_DIR/system.iso $SYSROOT

if [ ${2:-_} = "run" ]; then
	qemu-system-i386 $TARGET_DIR/system.iso
fi
