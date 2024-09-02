# Write a nbdkit plugin in Rust

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
