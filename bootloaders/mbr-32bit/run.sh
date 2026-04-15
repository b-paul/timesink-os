#!/bin/sh

BUILD_DIR=$(dirname $1)
IMAGE_BIN="$BUILD_DIR/image.bin"

./build.sh $1 $IMAGE_BIN || exit

qemu-system-i386 -hda $IMAGE_BIN
