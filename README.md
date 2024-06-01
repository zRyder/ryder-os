# RyderOS

RyderOS is a bare metal operating system written entirely in Rust.
___

## Building
RyderOS is build on Rust version 1.78.0-nightly and is the recommended 
version building RyderOS.

### Specifying Target
RyderOS is built on the bare metal environment `x86_64-unknown-none`. This environment will describe an 
[embedded](https://en.wikipedia.org/wiki/Embedded_system), and 
[ARM](https://en.wikipedia.org/wiki/ARM_architecture_family) based system. This environment
will have no underlying Operating System which makes it perfect to build RyderOS on top of.

RyderOs can be built by running the following command:
```shell
cargo build --target x86_64-unknown-uefi
```

This target is also specified as the default target in `ryder_os_kernel/.cargo/config.toml` so running `cargo build`
with no `--target` flag is also valid.

---
## Running

RyderOS can be run via virtual machine, like [QEMU](https://www.qemu.org/download/#source), or from real hardware. 
You can easily run with QEMU by simply running the following commands

Since RyderOS is built with the UEFI standard, a BIOS image can be produced alongside a UEFI image.
To build these, run the outermost component of the Operating System located at `ryder_os`. You can
run the following commands.

```shell
cargo run --package ryder_os --bin ryder_os -- uefi
```
*To run the UEFI image*

```shell
cargo run --package ryder_os --bin ryder_os
```
*To run the BIOS image*

If you wish to run on a real piece of hardware or 
another VM provider, after building a bootable image will exist in
`target/{BUILD_RELEASE}/uefi.img` and `target/{BUILD_RELEASE}/bios.img`.
This image can be directly booted off of. Please note that not knowing what you're
doing can result in loss of data if you chose this method. For this reason, QEMU is recommended.
