#!/bin/bash

set -x

DIR="$(realpath $(dirname "$0"))"

KERNEL_SRC="${DIR}/linux-6.10.6"
BUSYBOX_SRC="${DIR}/busybox-1.36.1"
INITRAMFS="${DIR}/initramfs.cpio.gz"

pushd ${KERNEL_SRC}
make -j 8
popd

pushd ${BUSYBOX_SRC}/_install
find . -print0 | cpio --null -o -v -H newc | gzip -9 >${INITRAMFS}
popd

qemu-system-x86_64 \
	-nographic \
	-kernel ${KERNEL_SRC}/arch/x86/boot/bzImage \
	-initrd ${INITRAMFS} \
	-append 'loglevel=15 console=ttyS0 init=/bin/sh'
