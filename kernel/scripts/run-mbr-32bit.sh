#!/bin/sh

BUILD_DIR=$(dirname $1)
IMAGE_BIN="$BUILD_DIR/image.bin"

./scripts/build-mbr-32bit.sh $1 || exit

qemu-system-i386 -hda ../target/image.bin
