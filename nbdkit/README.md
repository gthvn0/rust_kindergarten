# Write a nbdkit plugin in Rust

- [nbdkit-plugin](https://libguestfs.org/nbdkit-plugin.3.html)

- Try it in C
  - We will create a plugin that will only write "hello" and that is backed by memory
  - To build the plugin: `make`
  - To start the nbdkit server:
    - To load it: `nbdkit -fv ./myplugin.so`
    - This should listen on localhost on port 10809
  - To connect from a client:
    - ensure that nbd module is loaded: `sudo modprobe nbd`
    - connect: `sudo nbd-client localhost 10809 /dev/nbd0`
    - Now read the first bytes: `dd if=/dev/nbd0 of=./read.bin bs=16 count=1`
    - Check that it is working: `hexdump -C ./read.bin`
    - deconnect: `sudo nbd-client -d /dev/nbd0`

- Then in Rust
  - It creates an empty disk with a specific header...
  - In the Rust implementation if everything works as expected you should be able to read the disk using `dd` and see:
  ```
  ❯ hexdump -C read.bin
00000000  48 65 6c 6c 6f 2c 20 53  61 69 6c 6f 72 21 00 00  |Hello, Sailor!..|
00000010  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
00000020
```
