# Links

- [Rust for Linux](https://rust-for-linux.com/)
- [Linux source](https://www.kernel.org/)
- [Busybox source](https://www.busybox.net/)

# Steps

- [x] Start by building a minimal linux kernel (tinyconf)
  - we use linux-6.10.6
- [x] Boot it using qemu (so no rootfs) and get the input on serial
- [ ] Build busybox, create initramfs and use it to boot shell.
  - we use busybox-1.36.1
