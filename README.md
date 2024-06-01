# RyderOS

RyderOS is a bare metal operating system written entirely in Rust.
___

## Building
RyderOS is build on Rust version 1.77.0-nightly and is the recommended 
version building RyderOS. There are two methods to building RyderOS.

### Method 1: Specifying Target
RyderOS is built on the bare metal environment. This environment will describe an 
[embedded](https://en.wikipedia.org/wiki/Embedded_system), and 
[ARM](https://en.wikipedia.org/wiki/ARM_architecture_family) based system. This environment
will have no underlying Operating System which makes it perfect to build RyderOS on top of.

The target is defined at `[x86_64-ryder_os.json](x86_64-ryder_os.json)` and RyderOs can be built
with this target by running the following command:
```shell
cargo build --target x86_64-ryder_os.json
```

This target is also specified as the default target in `.cargo/config.toml` so running `cargo build`
with no `--target` flag is also valid.

### Method 2: Linker Arguments
This method can be tougher for those who aren't familiar with linkers and as such it is not
recommended to use this method. This method is dependent on the OS running on your working
system.

#### MacOS

Run the following command with the specified linker arguments:
```shell
cargo rustc -- -C link-args="-e __start -static"
```

- `-e __start`: - This is the entry point, which is named `_start`. MacOS prefixes all function
names with an underscore (_) so make sure to include the extra underscore.
- `-static`: - macOS does not support statically linked libraries and will link to `libSystem` by default.
This argument overrides this and allows us to use a static binary.
---
## Running

RyderOS can be run via virtual machine, like [QEMU](https://www.qemu.org/download/#source), or from a non UEFI hardware. 
You can easily run with QEMU by simply running `cargo run`. If you wish to run on a real piece of hardware or 
another VM provider, after building a bootable image will exist in `target/x86_64-ryder_os/{BUILD_RELEASE}/build/bootimage-ryder_os.bin`.
This image can be directly booted off of.
